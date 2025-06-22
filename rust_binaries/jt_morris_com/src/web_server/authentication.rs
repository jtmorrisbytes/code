#![allow(warnings)]
use crate::frontend::net::{
    FinishWebAuthnRegistrationResponseErrorKind, StartWebAuthnRegistrationError,
};
use std::collections::HashMap;
// use diesel::prelude::{Insertable, Queryable};
// use serde::{Deserialize, Serialize};

use base64::Engine;

use uuid::Uuid;
// use diesel::prelude::Insertable;
use crate::frontend::net::{
    FinishWebAuthnRegistrationBody, FinishWebAuthnRegistrationResponseBody,
    StartWebAuthnRegistrationResponse, StartWebAuthnRegistrationResponseBody,
};
use rocket::{
    http::Status,
    response::content::{RawHtml, RawMsgPack},
    serde::msgpack::MsgPack,
    State,
};
use webauthn_rs::{prelude::PasskeyRegistration, Webauthn};
// use webauthn_rs::prelude::CreationChallengeResponse;

// #[derive()]
// #[diesel(table_name=schema::passkey_registration_states)]
pub struct PasskeyRegistrationState {
    // id: uuid::Uuid,
    username: String,
    user_id: Uuid,
    passkey_registration: PasskeyRegistration,
}
// #[derive(Serialize,Deserialize,Queryable,Insertable)]
// #[diesel(table_name=schema::users_passkeys)]
pub struct UsersPasskeys {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    credential_id: Vec<u8>,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Default)]
pub struct PasskeyRegistrationStateMemory(
    rocket::tokio::sync::Mutex<HashMap<Uuid, PasskeyRegistrationState>>,
);

// WEBAUTHN

// use webauthn_rs::prelude::*;

pub const WEBAUTHN_RELYING_PARTY_ID: &str = "webauthn.relying_party_id";
pub const WEBAUTHN_RELYING_PARTY_PORT: &str = "webauthn.relying_party_port";
pub const WEBAUTHN_RELYING_PARTY_URL: &str = "webauthn.relying_party_url";
pub const WEBAUTHN_RELYING_PARTY_NAME: &str = "webauthn.relying_party_name";

pub struct WebAuthnFairing;

#[rocket::async_trait]
impl rocket::fairing::Fairing for WebAuthnFairing {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "WebAuthn Fairing",
            kind: rocket::fairing::Kind::Ignite,
        }
    }
    async fn on_ignite(&self, rocket: rocket::Rocket<rocket::Build>) -> rocket::fairing::Result {
        let figment = rocket.figment();
        // attempt to extract the relying party url from figment
        let mut rp_url: url::Url = match figment.extract_inner(WEBAUTHN_RELYING_PARTY_URL) {
            Ok(rp_url) => rp_url,
            Err(e) => {
                tracing::error!("Failed to extract rp_url from figment {e}");
                return Err(rocket);
            }
        };

        if rp_url.domain().is_none() {
            tracing::error!(
                "Expected fully qualified domain name in public_base_url for WebAuthN Confiuration"
            );
            return Err(rocket);
        }
        if rp_url.domain() != Some("localhost") {
            rp_url.set_port(None).ok();
        }
        rp_url.set_query(None);
        rp_url.set_path("");
        let rp_id: Option<&str> = figment
            .extract_inner(WEBAUTHN_RELYING_PARTY_ID)
            .ok()
            .flatten()
            .or_else(|| rp_url.domain());
        let rp_id = match rp_id {
            Some(rp_id) => rp_id,
            None => {
                tracing::error!("Failed to determine relying_party_id. check configuration for webauthn.relying_party_id or configure a domain name in {WEBAUTHN_RELYING_PARTY_URL} ");
                return Err(rocket);
            }
        };
        let mut webauthn = match webauthn_rs::WebauthnBuilder::new(rp_id, &rp_url) {
            Ok(w) => w,
            Err(e) => {
                tracing::error!("Failed to create webauthn {e}");
                return Err(rocket);
            }
        };
        match figment.extract_inner::<Option<&str>>(WEBAUTHN_RELYING_PARTY_NAME) {
            Ok(Some(name)) => webauthn = webauthn.rp_name(name),
            _ => {}
        }

        let webauthn = match webauthn.build() {
            Ok(w) => w,
            Err(e) => {
                tracing::error!("Failed to build webauthn {e}");
                return Err(rocket);
            }
        };
        Ok(rocket.manage(webauthn))
    }
}

// use crate::frontend::FrontendTemplatePath;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::get("/authentication/start.html?<username>", format = "text/html")]
pub async fn start_login(
    template_path: super::ssr::FrontendTemplatePath,
    username: Option<String>,
    // registration_state: &State<PasskeyRegistrationStateMemory>
) -> Result<(Status, RawHtml<String>), Status> {
    /*
        here we are server side rendering a yew frontend web app. somehow we need the client to be able to know what its inital state should be
        when the page loads.

        we create an instance of frontend::FrontendAppProperties and serialize it to msgpack. we could use json here b

    */
    let route = crate::frontend::Route::StartAuthentication {
        ccr_url: rocket::uri!(start_webauthn_registration(Option::<String>::None)).to_string(),
        username,
    };
    let title: &str = route.title();
    let properties: crate::frontend::FrontendAppProperties =
        crate::frontend::FrontendAppProperties {
            title: title.to_string(),
            route,
        };

    let csr_initial_data_bytes = rocket::serde::msgpack::to_vec(&properties).unwrap();
    let initial_data = base64::prelude::BASE64_STANDARD.encode(&csr_initial_data_bytes);
    let html = super::ssr::server_render(template_path, properties, initial_data)
        .await
        .map_err(|e| {
            tracing::error!("{e}");
            Status::InternalServerError
        })?;
    Ok((Status::Ok, RawHtml(html)))
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::post(
    "/authentication/start-webauthn-registration?<username>",
    format = "application/msgpack"
)]
pub async fn start_webauthn_registration(
    mut connection: super::db::PrimaryDatabasePoolConnection,
    webauthn: &State<Webauthn>,
    registration_state: &State<PasskeyRegistrationStateMemory>,
    username: Option<String>,
    // always respond with 200 code unless this function panics
) -> Result<RawMsgPack<Vec<u8>>, Status> {
    use super::db::schema;
    // use diesel::prelude::*;
    use super::db::diesel;
    use super::db::diesel_async;
    use diesel::{ExpressionMethods, QueryDsl};
    let retval: StartWebAuthnRegistrationResponseBody = async {
        if username.is_none() {
            return StartWebAuthnRegistrationResponseBody::Err(
                StartWebAuthnRegistrationError::MissingUsername,
            );
        }

        use diesel_async::RunQueryDsl;
        // attempts to find the user from a username
        let username = username.unwrap();
        let query = schema::users::table
            .select(schema::users::id)
            .filter(schema::users::username.eq(&username));
        let query_result = query.get_result(&mut connection).await;

        let user_id = match query_result {
            Ok(u) => u,
            Err(diesel::result::Error::NotFound) => uuid::Uuid::new_v4(),
            Err(e) => {
                tracing::error!("{e}");
                return StartWebAuthnRegistrationResponseBody::Err(
                    StartWebAuthnRegistrationError::UsernameLookupFailed(e.to_string()),
                );
            }
        };
        let (ccr, _reg_state) =
            match webauthn.start_passkey_registration(user_id, &username, &username, None) {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!("start passkey authentication failed {e}");
                    return StartWebAuthnRegistrationResponseBody::Err(
                        StartWebAuthnRegistrationError::WebAuthnError(e.to_string()),
                    );
                }
            };
        let passkey_state_id = Uuid::new_v4();
        let passkey_state: PasskeyRegistrationState = PasskeyRegistrationState {
            user_id,
            username,
            passkey_registration: _reg_state,
        };
        {
            let mut lock = registration_state.0.lock().await;
            let _ = lock.insert(passkey_state_id, passkey_state);
        }
        StartWebAuthnRegistrationResponseBody::Ok(StartWebAuthnRegistrationResponse {
            passkey_state_id,
            ccr,
        })
    }
    .await;
    rmp_serde::to_vec_named(&retval)
        .map(|b| RawMsgPack(b))
        .map_err(|e| {
            tracing::error!("{e}");
            Status::InternalServerError
        })
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::post(
    "/authentication/finish-webauthn-registration",
    data = "<body>",
    format = "msgpack"
)]
pub async fn finish_webauthn_registration(
    body: MsgPack<FinishWebAuthnRegistrationBody>,
    registration_state: &State<PasskeyRegistrationStateMemory>,
    webauthn: &State<Webauthn>,
) -> MsgPack<FinishWebAuthnRegistrationResponseBody> {
    // use schema;
    // use diesel::prelude::*;

    let passkey_state = {
        let mut lock = registration_state.0.lock().await;
        lock.remove(&body.passkey_state_id)
    };

    if passkey_state.is_none() {
        // invaid uuid or repeated request
        return MsgPack(FinishWebAuthnRegistrationResponseBody::Err(
            FinishWebAuthnRegistrationResponseErrorKind::InvalidPasskeyState,
        ));
    }
    let passkey_state = passkey_state.unwrap();

    // get the users credentials
    let passkey = match webauthn.finish_passkey_registration(
        &body.public_key_request,
        &passkey_state.passkey_registration,
    ) {
        Ok(passkey) => passkey,
        Err(webauthn_error) => {
            tracing::error!("{webauthn_error}");
            return MsgPack(FinishWebAuthnRegistrationResponseBody::Err(
                FinishWebAuthnRegistrationResponseErrorKind::WebAuthnError(
                    webauthn_error.to_string(),
                ),
            ));
        }
    };
    // webauthn says that we need to check if this credential id is registered to another user
    passkey.cred_id();
    // schema::users_passkeys
    MsgPack(FinishWebAuthnRegistrationResponseBody::Ok(()))
}
