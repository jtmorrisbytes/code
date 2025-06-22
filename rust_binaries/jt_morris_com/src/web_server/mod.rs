pub use db;
// pub mod auth_state;
pub mod authentication;
pub mod authorization;
pub mod config;
pub mod ops;
pub mod registration;
pub mod response;
pub mod session;
pub mod url;
pub mod user;
pub mod vehicles;
// pub mod schema;
pub mod ssr;

use authentication::PasskeyRegistrationStateMemory;
use authentication::WebAuthnFairing;
use config::ServerConfig;
use db::PrimaryDatabasePool;
use response::{HtmlResponse, InternalServerError, UnauthorizedError};
pub use rocket::tokio;
use ssr::FrontendTemplatePathFairing;

pub use authorization::AUTH0_BASE_SCOPES;

// use webauthn_rs::prelude::*;

// use sqlx::Connection;
use time::format_description::well_known::Rfc3339;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::catch(401)]
pub async fn catch_unauthorized(_request: &rocket::Request<'_>) -> HtmlResponse {
    HtmlResponse::unauthorized(UnauthorizedError::default())
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::catch(500)]
pub async fn catch_internal_server_error(_request: &rocket::Request<'_>) -> HtmlResponse {
    HtmlResponse::internal_server_error(InternalServerError::default())
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::get("/", format = "text/html")]
pub async fn render_index(
    // connection: rocket_db_pools::Connection<PrimaryDatabase>,
    server_config: &rocket::State<config::ServerConfig>,
    access_token: Option<authorization::AccessToken>,
    public_base_uri: RocketAbsoluteBaseUri,
) -> Result<rocket_dyn_templates::Template, rocket_dyn_templates::Template> {
    let next_url = access_token
        .map(|_| {
            rocket::uri!(
                (*public_base_uri).to_owned(),
                user::render_user_dashboard_template()
            )
        })
        .unwrap_or_else(|| {
            rocket::uri!(
                (*public_base_uri).to_owned(),
                authorization::authorize(Option::<String>::None, Option::<String>::None)
            )
        });
    Ok(rocket_dyn_templates::Template::render(
        "main_landing_page",
        rocket_dyn_templates::context! {next_url,support_email:server_config.support_email()},
    ))
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub async fn rocket() -> Result<rocket::Rocket<rocket::Build>, Box<dyn std::error::Error>> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_timer(tracing_subscriber::fmt::time::UtcTime::<Rfc3339>::rfc_3339())
        .init();

    // we are only taking in one optional argument. assume that the first argument is a path to an env file.
    // we are trying to avoid a dependency on clap at this time

    if let Some(arg_env_path) = std::env::args()
        .nth(1)
        .map(|arg| std::path::PathBuf::from(arg))
    {
        let _ = dotenvy::from_path(arg_env_path)
            .map_err(|e| tracing::error!("{e}"))
            .ok();
    } else {
        let _ = dotenvy::dotenv().map_err(|e| tracing::error!("{e}")).ok();
    }

    // perform any startup tasks neeeded before starting the webserver
    // create the primary database pool
    let mut rocket = rocket::Rocket::build();
    let figment = rocket.figment().to_owned();

    let primary_database_pool = PrimaryDatabasePool::try_new_from_figment(&figment).await?;

    tracing::info!("configuring server");

    let server_config = config::ServerConfig::try_new_from_figment(&figment)?;
    tracing::debug!("configuring authentication");
    // let env_client_secret = std::env::var("ROCKET_AUTH0_CLIENT_SECRET").or_else(|_e| std::env::var("AUTH0_CLIENT_SECRET")).expect("AUTH0_CLIENT_SECRET or ROCKET_AUTH0_CLIENT_SECRET env variables");
    let authentication_client_options = auth0::AuthenticationClientOptions {
        client_id: server_config.auth0_client_id().to_string(),
        client_secret: Some(server_config.auth0_client_secret().to_owned()),
        auth0_domain: server_config.auth0_domain().to_string(),
        jwks: None,
        jwt_algorythm: None,
    };
    let authentication_client_backoff =
        authorization::AuthenticationClientBackoff(tokio::sync::Mutex::new(0));
    let authentication_client =
        auth0::AuthenticationClient::try_initialize(authentication_client_options).await?;
    rocket = rocket
        .mount(
            "/",
            rocket::fs::FileServer::from(server_config.public_files_root()),
        )
        // .mount("/", authentication::routes())
        // .manage(pool)
        .manage(PasskeyRegistrationStateMemory::default())
        .manage(server_config)
        // .manage(public_base_url)
        .manage(authentication_client)
        .manage(primary_database_pool)
        // .manage(s3_client)
        .manage(authentication_client_backoff)
        .register(
            "/",
            rocket::catchers![catch_internal_server_error, catch_unauthorized],
        );
    #[allow(unused_mut)]
    let mut shield = rocket::shield::Shield::default();
    #[cfg(feature = "tls")]
    {
        shield = shield.enable(rocket::shield::Hsts::Preload(time::Duration::days(365)));
    }
    rocket = rocket
        .attach(shield)
        .attach(WebAuthnFairing)
        .attach(FrontendTemplatePathFairing)
        .mount(
            "/",
            rocket::routes![
                render_index,
                user::render_user_dashboard_template,
                user::render_user_settings_template,
                user::render_transactions_upload_template,
                user::upload_user_transactions_from_csv,
                ops::render_ops_index,
                // ops::render_ops_dbquery_interface,
                ops::render_ops_users_list,
                authentication::start_login,
                authentication::start_webauthn_registration,
                authentication::finish_webauthn_registration,
                authorization::authorize,
                authorization::handle_auth_callback,
            ],
        )
        .attach(rocket_dyn_templates::Template::fairing());
    Ok(rocket)
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(Clone)]
pub struct RocketAbsoluteBaseUri(pub rocket::http::uri::Absolute<'static>);
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl std::ops::Deref for RocketAbsoluteBaseUri {
    type Target = rocket::http::uri::Absolute<'static>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub type ManagedPoolState<'a> = &'a rocket::State<db::PgPool>;
// pub type ManagedPoolsState<'a> = &'a rocket::State<HashMap<String, crate::server::db::PgPool>>;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for RocketAbsoluteBaseUri {
    type Error = anyhow::Error;
    async fn from_request(
        r: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let uri = r
            .local_cache_async(async {
                // let default = proc_macros::make_absolute_rocket_uri!();
                let figment = r.rocket().figment();
                // let server_config =

                let config = ServerConfig::try_new_from_figment(figment).unwrap();
                rocket::http::uri::Absolute::<'static>::parse_owned(
                    config.public_base_url().to_string(),
                )
                .unwrap()
            })
            .await;
        rocket::request::Outcome::Success(Self(uri.to_owned()))
    }
}
