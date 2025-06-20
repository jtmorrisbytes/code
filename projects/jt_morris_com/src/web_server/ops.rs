use super::authorization::{
    AccessToken, AccessTokenRequestError, AUTH0_BASE_SCOPES, SCOPE_ACCESS_OPS,
};
use proc_macros::IntoTemplate;
// use rocket::Request;
// use rocket_db_pools::diesel::prelude::RunQueryDsl;
// use rocket_dyn_templates::Template;

pub struct AccessOpsToken(AccessToken);
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for AccessOpsToken {
    type Error = anyhow::Error;
    async fn from_request(
        r: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let t: AccessToken = match r.guard::<AccessToken>().await {
            rocket::request::Outcome::Success(a_t) => a_t,
            rocket::request::Outcome::Error((status, error)) => {
                return rocket::request::Outcome::Error((status, anyhow::Error::new(error)))
            }
            rocket::request::Outcome::Forward(e) => return rocket::request::Outcome::Forward(e),
        };
        if !t
            .claims()
            .permissions
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .contains(&SCOPE_ACCESS_OPS)
        {
            return rocket::outcome::Outcome::Error((
                rocket::http::Status::Unauthorized,
                anyhow::Error::msg(format!(
                    "Missing required scope {} in access token",
                    SCOPE_ACCESS_OPS
                )),
            ));
        }
        rocket::request::Outcome::Success(AccessOpsToken(t))
    }
}
impl std::ops::Deref for AccessOpsToken {
    type Target = AccessToken;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// use crate::PrimaryDatabase;
//

// /// This is a permmission string that will allow access to the ops application
// #[cfg(not(all(target_arch="wasm32",target_os="unknown")))]
// pub fn reauthorize_on_access_token_error<'r>(
//     access_token_result: Result<AccessToken, &'r str>,
//     // server_config: &crate::ServerConfig,
// ) -> Result<AccessToken, rocket::response::Redirect> {
//     if access_token_result.is_err() {
//         let access_token_error_message = access_token_result.unwrap_err();
//         tracing::debug!("{access_token_error_message}");
//         return Err(rocket::response::Redirect::to(rocket::uri!(
//             super::authorization::authorize(Some(SCOPE_ACCESS_OPS), Option::<String>::None,)
//         )));
//     } else {
//         Ok(access_token_result.unwrap())
//     }
// }
// #[cfg(not(all(target_arch="wasm32",target_os="unknown")))]

// pub fn reauthorize_when_access_token_does_not_contain_access_ops_scope(
//     access_token: &AccessToken,
//     // server_config: &crate::ServerConfig,
// ) -> Result<(), rocket::response::Redirect> {
//     if !access_token.permissions().contains(&SCOPE_ACCESS_OPS) {
//         tracing::debug!("Access token missing required scope {SCOPE_ACCESS_OPS}.");
//         let mut scopes = access_token.permissions().clone();
//         scopes.push(SCOPE_ACCESS_OPS);
//         let scopes = scopes.join(",");
//         Err(rocket::response::Redirect::to(rocket::uri!(
//             crate::authorization::authorize(
//                 Some(scopes),
//                 Option::<String>::None // server_config.auth0_audience()
//             )
//         )))
//     } else {
//         Ok(())
//     }
// }

#[derive(serde::Serialize, IntoTemplate)]
pub struct RenderOpsIndexContext {
    ops_list_users_href: String,
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::get("/ops", format = "text/html")]
pub async fn render_ops_index(
    public_base_uri: super::RocketAbsoluteBaseUri,
    // connection: crate::PrimaryDatabasePool,
    // authentication_client_backoff: &State<crate::authorization::AuthenticationClientBackoff>,
    access_token_result: Result<AccessToken, AccessTokenRequestError>,
    // authentication_client: &State<AuthenticationClient>,
    // server_config: &rocket::State<crate::ServerConfig>,
) -> Result<super::response::HtmlResponse, rocket::response::Redirect> {
    let return_uri = rocket::uri!((*public_base_uri).to_owned(), render_ops_index()).to_string();
    let return_url = url::Url::parse(&return_uri).unwrap();
    let access_token = match access_token_result {
        Err(error) => {
            tracing::error!("{error}");
            // call the authorize endpoint directly, allowing to pass the return url here for 'automagic' redirect!
            // we were not able to determine what scopes are in the token, because the token expired. ask for the minimum scopes required here
            let scope = AUTH0_BASE_SCOPES.to_string() + " " + SCOPE_ACCESS_OPS;
            // render the authorize page directly, this allows us to return here without redirecting
            return Err(rocket::response::Redirect::to(rocket::uri!(
                super::authorization::authorize(
                    Some(scope),
                    Some(super::url::Url::from(return_url))
                )
            )));
        }
        Ok(token) => token,
    };

    // at this point the user will have permisson to obtain a token of this type or the authorization endpoint will try to obtain a token with this scope that the user will not be allowed to have, terminating with an error;
    // as you can ask for a permission but not be allowed to have it.
    if !access_token.permissions().contains(&SCOPE_ACCESS_OPS) {
        let mut s = access_token.permissions().clone();
        s.push(&SCOPE_ACCESS_OPS);
        let scope = s.join(" ");
        return Err(rocket::response::Redirect::to(rocket::uri!(
            super::authorization::authorize(Some(scope), Some(super::url::Url::from(return_url)))
        )));
    }
    Ok(super::response::HtmlResponse::success_into_template(
        RenderOpsIndexContext {
            ops_list_users_href: rocket::uri!(render_ops_users_list(
                Option::<u32>::None,
                Option::<u32>::None
            ))
            .to_string(),
        },
    ))
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::get("/ops/users.list.html?<limit>&<page>", format = "text/html")]
pub async fn render_ops_users_list(
    // pool: crate::PrimaryDatabaseConnection,
    #[allow(unused)] access_token_result: Result<AccessToken, AccessTokenRequestError>,
    // server_config: &rocket::State<crate::ServerConfig>,
    #[allow(unused)] limit: Option<u32>,
    #[allow(unused)] page: Option<u32>,
) -> Result<rocket_dyn_templates::Template, rocket::response::Redirect> {
    todo!("ops.users.list");
}
