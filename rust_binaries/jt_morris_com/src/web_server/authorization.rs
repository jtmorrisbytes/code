use serde::{Deserialize, Serialize};
use time;
pub const AUTH0_BASE_SCOPES: &'static str = "openid email profile";
pub const SCOPE_ACCESS_OPS: &'static str = "access:ops";
pub const SCOPE_READ_OTHERS_DATA: &str = "read:others";
pub const SCOPE_UPDATE_OTHERS_DATA: &str = "update:others";
pub const SCOPE_DELETE_OTHERS_DATA: &str = "delete:others";
pub const SCOPE_CREATE_OTHERS_DATA: &str = "create:others";
// NOTE: use crate::PrimaryDatabaseConnection instead of rocket_db_pools::Connection<crate::PrimaryDatabase> for tls workaround!
// use crate::PrimaryDatabase;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::get("/")]
pub async fn render_index() {}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub fn render_error_template(
    next_uri: &rocket::http::uri::Absolute<'_>,
    // title: &str,
    // heading: &str,
    message: Option<&str>,
    error: impl std::convert::Into<Box<dyn std::error::Error>>,
) -> rocket_dyn_templates::Template {
    let error: Box<dyn std::error::Error> = error.into();
    rocket_dyn_templates::Template::render(
        "error",
        rocket_dyn_templates::context! {
             next_uri: next_uri.to_string(),
             title: "Server Error",
             heading: "Server Error",
             message:format!("A General Server Error Occurred: {}",message.unwrap_or("No further context was provided")),
             error: error.to_string()
        },
    )
}

// use sqlx::any;

// use crate::render_error_template;

/// a wrapper struct that 'remembers' how long to delay external authentication requests for
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub struct AuthenticationClientBackoff(pub rocket::tokio::sync::Mutex<u64>);
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl std::default::Default for AuthenticationClientBackoff {
    fn default() -> Self {
        AuthenticationClientBackoff(rocket::tokio::sync::Mutex::new(1))
    }
}

// Currently authentication is handled by a third party service
// use crate::AUTH0_APP_BASE_SCOPES as BASE_SCOPE;
#[derive(Deserialize, Debug, PartialEq)]
pub struct Claims {
    pub sub: String,
    pub email: Option<String>,
    pub iat: Option<i64>,
    pub exp: Option<i32>,
    pub permissions: Vec<String>,
}
impl Claims {
    pub fn expires_utc(&self) -> time::OffsetDateTime {
        time::OffsetDateTime::from_unix_timestamp(self.exp.unwrap_or(0).into())
            .expect("Getting an OffsetDateTime from the exp claim should not fail")
    }
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub struct AccessTokenCookie<'a>(pub rocket::http::Cookie<'a>);

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<'a> AccessTokenCookie<'a> {
    pub fn create_cookie(
        domain: String,
        path: String,
        expires: time::OffsetDateTime,
        access_token: String,
    ) -> rocket::http::Cookie<'a> {
        let mut cookie = rocket::http::Cookie::new("access_token", access_token);
        cookie.set_http_only(Some(true));
        cookie.set_secure(Some(true));
        cookie.set_same_site(Some(rocket::http::SameSite::Lax));
        cookie.set_domain(domain);
        cookie.set_path(path);
        cookie.set_expires(Some(expires));

        cookie
    }
}
impl<'a> AccessTokenCookie<'a> {
    pub fn inner(self) -> rocket::http::Cookie<'a> {
        self.0
    }
}
#[derive(thiserror::Error, Debug)]
#[error("Access Token Error: {0}")]
pub enum AccessTokenRequestError {
    #[error("Access token cookie not present in cookie jar")]
    CookieNotPresent,
    #[error("Authentication client is not available in rocket state. Make sure you have configured server correctly!")]
    AuthenticationClientNotAvailable,
    #[error("Failed to verify access token: {underlying_error}")]
    FailedToVerifyAccessToken { underlying_error: String },
    #[error("An error occurred while trying to get an OffsetDateTime from the expiry claims from a provided access token: {underlying_error}")]
    FailedToParseOffsetDateTimeFromAccessToken { underlying_error: String },
    #[error("Token Expired")]
    AccessTokenExpired,
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for AccessToken {
    type Error = AccessTokenRequestError;
    async fn from_request(
        req: &'r rocket::request::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let server_config = req.rocket().state::<super::config::ServerConfig>().unwrap();
        let access_token_cookie = req.cookies().get_private("access_token");
        if access_token_cookie.is_none() {
            tracing::debug!("{:?}", AccessTokenRequestError::CookieNotPresent);
            return rocket::request::Outcome::Error((
                rocket::http::Status::Unauthorized,
                AccessTokenRequestError::CookieNotPresent,
            ));
        }
        let access_token_cookie = access_token_cookie.unwrap();
        let authentication_client_guard_outcome = req
            .guard::<&rocket::State<auth0::AuthenticationClient>>()
            .await;
        if authentication_client_guard_outcome.is_error() {
            return rocket::request::Outcome::Error((
                rocket::http::Status::InternalServerError,
                AccessTokenRequestError::AuthenticationClientNotAvailable,
            ));
        }
        let authentication_client = authentication_client_guard_outcome.unwrap();
        // .expect("Authentication Client should be available in state");
        let token_verification_result = authentication_client
            .verify_token::<Claims>(access_token_cookie.value(), server_config.auth0_audience());
        if token_verification_result.is_err() {
            let underlying_error = token_verification_result.unwrap_err().to_string();
            let error = AccessTokenRequestError::FailedToVerifyAccessToken { underlying_error };
            tracing::debug!("{}", error);
            return rocket::request::Outcome::Error((rocket::http::Status::Unauthorized, error));
        }
        let token = token_verification_result.unwrap();
        // check the expiry
        let expiry_i32 = token.claims().exp.unwrap_or_default();
        let expiry_datetime = match time::OffsetDateTime::from_unix_timestamp(expiry_i32.into()) {
            Ok(d) => d,
            Err(e) => {
                let error = AccessTokenRequestError::FailedToParseOffsetDateTimeFromAccessToken {
                    underlying_error: e.to_string(),
                };
                tracing::error!("{}", error);
                return rocket::request::Outcome::Error((
                    rocket::http::Status::InternalServerError,
                    error,
                ));
            }
        };
        let now = time::OffsetDateTime::now_utc();
        if now > expiry_datetime {
            let error = AccessTokenRequestError::AccessTokenExpired;
            tracing::debug!("{}", error);
            return rocket::request::Outcome::Error((
                rocket::http::Status::InternalServerError,
                error,
            ));
        }
        rocket::request::Outcome::Success(Self(token))
    }
}
#[derive(Debug)]
pub struct AccessToken(
    auth0::AccessToken<Claims>,
    // expires: time::OffsetDateTime
);
impl AccessToken {
    pub fn claims(&self) -> &Claims {
        self.0.claims()
    }
    pub fn value(&self) -> &str {
        self.0.access_token()
    }
    pub fn permissions(&self) -> Vec<&str> {
        self.0
            .claims()
            .permissions
            .iter()
            .map(|s| s.as_str())
            .collect()
    }
    pub fn inner(&self) -> &auth0::AccessToken<Claims> {
        &self.0
    }
}

/// retains some information used while authenticating a request
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AuthenticationState {
    pub id: uuid::Uuid,
    pub started: time::OffsetDateTime,
    pub return_url: Option<String>,
    pub redirect_url: String,
    pub scope: String,
}
// pub fn parse_url
#[derive(serde::Deserialize)]
pub struct UserInfo {
    name: String,
    given_name: Option<String>,
    family_name: Option<String>,
    profile: Option<String>,
    picture: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    locale: Option<String>,
}
impl UserInfo {
    pub fn full_name(&self) -> &str {
        &self.name
    }
    pub fn given_name(&self) -> Option<&str> {
        self.given_name.as_deref()
    }
    pub fn family_name(&self) -> Option<&str> {
        self.family_name.as_deref()
    }
    pub fn profile(&self) -> Option<&str> {
        self.profile.as_deref()
    }
    pub fn picture(&self) -> Option<&str> {
        self.picture.as_deref()
    }
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
    pub fn phone_number(&self) -> Option<&str> {
        self.phone_number.as_deref()
    }
    pub fn locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::get("/auth/callback?<state>&<code>&<error>&<error_description>", rank = 1)]
pub async fn handle_auth_callback<'r>(
    // mut connection: super::db::PrimaryDatabasePoolConnection,
    pool: super::db::PrimaryDatabasePool,
    authentication_client: &rocket::State<auth0::AuthenticationClient>,
    authentication_client_backoff: &rocket::State<AuthenticationClientBackoff>,
    cookie_jar: &rocket::http::CookieJar<'_>,
    server_config: &rocket::State<super::config::ServerConfig>,
    public_base_uri: super::RocketAbsoluteBaseUri,
    state: Option<uuid::Uuid>,
    code: Option<&str>,
    error: Option<&str>,
    error_description: Option<&str>,
) -> Result<rocket::response::Redirect, super::response::HtmlResponse> {
    use super::response::{BadRequestError, HtmlResponse};
    let mut connection = pool.get().await.map_err(|e| e.to_string())?;

    let index_uri = rocket::uri!((*public_base_uri).to_owned(), super::render_index());

    let (code, state) = match (code, state, error, error_description) {
        (Some(code), Some(state), None, None) => (code, state),
        (None, Some(state), Some(error), Some(error_description)) => {
            // the authorization server sent an error as a callback
            // let connection = pool.get_connection()?;

            // let state = state.clone();
            // let query = AuthState::delete_by_id(&mut connection, &state);
            let _ = super::db::auth_state::AuthState::delete_by_id(&mut connection, &state).await?;

            return Err(super::response::HtmlResponse::unauthorized(
                super::response::UnauthorizedError {
                    error: format!(
                        "Authorization Error:{error} error_description:{error_description}"
                    ),
                    return_uri: index_uri.to_string().to_string(),
                },
            ));
        }
        _ => {
            return Err(super::response::HtmlResponse::bad_request(
                super::response::BadRequestError {
                    error: "Bad request expected code & state".to_string(),
                    return_uri: index_uri.to_string(),
                },
            ));
        }
    };
    // let mut connection = pool.get()?;
    // attempt to delete the auth state if it is found. if it is not found, returns NULL, then it has already been used
    // and the user should retry authentication
    let query_result =
        super::db::auth_state::AuthState::delete_by_id_returning_self(&mut connection, &state)
            .await;

    let auth_state = match query_result {
        Ok(state) => state,
        Err(error) => {
            return Err(HtmlResponse::bad_request(BadRequestError {
                error: format!("Database Error {state},{error}"),
                return_uri: index_uri.to_string(),
            }))
        }
    };
    tracing::info!("Attempting to exchange an authorization code for an authorization token");
    let access_token = authentication_client
        .exchange_code_for_token::<Claims>(
            &auth_state.scope,
            &auth_state.redirect_url,
            &state.to_string(),
            &code,
            &server_config.auth0_audience(),
            authentication_client_backoff.inner().0.lock().await.clone(),
        )
        .await?;
    tracing::info!("Attempting to exchange an access token for the users profile information");
    let user_info: UserInfo = authentication_client
        .get_user_info(&access_token, *authentication_client_backoff.0.lock().await)
        .await?;
    // since we have a valid access token at this point, we should create the user if they dont exist.
    // the user will be identified externally by the auth0 subject in the access token
    let email = user_info.email();
    let full_name = user_info.full_name();
    let picture: Option<&str> = user_info.picture();
    let profile: Option<&str> = user_info.profile();
    let phone_number: Option<&str> = user_info.phone_number();

    tracing::info!(
        "Attempting to create the user identified by the subject claim in their access token"
    );

    let pool_clone = pool.inner();
    let _ = super::db::user::User::create(pool_clone,
        access_token.claims().sub.as_str(),
        full_name,
        email.unwrap_or_default(),
        phone_number,
        picture,
        profile,
        access_token.claims().sub.clone()).await.map_err(|e| e.context("while attempting to insert/create a user as identified by the subject claim in their access token with information provided by the call to the userinfo endpoint"))?;
    // create an access token cookie and place it privately inside the cookie jar. We may be vonerable to replay attacks, but the attacker cannot simply copy & paste the token into auth0
    const ACCESS_TOKEN_PATH: &str = "/";
    let access_token_string = access_token.access_token().to_string();
    let authority = public_base_uri.authority().ok_or_else(|| HtmlResponse::internal_server_error(super::response::InternalServerError{error: "unable to determine the configured website authority while attempting to generate an access token cookie".to_string(),return_uri:index_uri.to_string()}))?;
    let access_token_cookie = AccessTokenCookie::create_cookie(
        authority.to_string(),
        ACCESS_TOKEN_PATH.to_string(),
        access_token.claims().expires_utc(),
        access_token_string,
    );
    cookie_jar.add_private(access_token_cookie);
    let return_url = auth_state
        .return_url
        .unwrap_or(rocket::uri!(super::render_index()).to_string());
    Ok(rocket::response::Redirect::to(return_url))
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::get("/authorize?<scope>&<return_url>", format = "text/html")]
pub async fn authorize(
    // NOTE: use crate::PrimaryDatabaseConnection instead of rocket_db_pools::Connection<crate::PrimaryDatabase> for tls workaround!
    pool: super::db::PrimaryDatabasePool,
    public_base_uri: super::RocketAbsoluteBaseUri,
    server_config: &rocket::State<super::config::ServerConfig>,
    authentication_client_backoff: &rocket::State<AuthenticationClientBackoff>,
    authentication_client: &rocket::State<auth0::AuthenticationClient>,
    scope: Option<String>,
    return_url: Option<crate::web_server::url::Url>,
    // audience: String,
) -> Result<rocket::response::Redirect, super::response::HtmlResponse> {
    let scope = scope.unwrap_or(AUTH0_BASE_SCOPES.to_string());
    let index_uri = rocket::uri!((*public_base_uri).to_owned(), super::render_index());
    let return_uri = return_url
        .as_deref()
        .map(|s: &url::Url| {
            rocket::http::uri::Absolute::<'_>::parse(s.as_str())
                .unwrap_or_else(|_| index_uri.clone())
        })
        .unwrap_or_else(|| index_uri.clone());
    // url unwrap_or_else go to the main page
    if public_base_uri.authority().as_ref().unwrap() != return_uri.authority().as_ref().unwrap() {
        return Err(super::response::HtmlResponse::bad_request(super::response::BadRequestError{error:"The server recived a request to redirect to an external website which is not supported.".to_string(),return_uri:index_uri.to_string()}));
    }

    let callback_url = rocket::uri!(
        (*public_base_uri).to_owned(),
        self::handle_auth_callback(
            Option::<uuid::Uuid>::None,
            Option::<String>::None,
            Option::<String>::None,
            Option::<String>::None
        )
    );
    let callback_url = url::Url::parse(&callback_url.to_string()).map_err(|e| {
        anyhow::Error::new(e)
            .context("while attempting to parse a callback url during a call to authorize")
    })?;
    let auth_state =
        super::db::auth_state::AuthState::new(return_url.as_deref(), scope.as_str(), &callback_url);

    let mut connection = pool.get().await.map_err(|e| anyhow::Error::new(e))?;
    let _ = auth_state.insert(&mut connection).await?;
    // let auth_state_uuid_create_result =
    let options = auth0::StartLoginWithRedirectOptions {
        scope: &scope,
        redirect_uri: callback_url.as_str(),
        audience: server_config.auth0_audience(),
        state: &auth_state.id.to_string(),
        connection: "",
        additional_parameters: Default::default(),
    };
    let next_url = authentication_client
        .start_login_with_redirect(
            &options,
            authentication_client_backoff.0.lock().await.clone(),
        )
        .map_err(|e| e.context("While attempting to generate a redirect url to auth0."))?;
    Ok(rocket::response::Redirect::to(next_url.to_string()))
}
#[rocket::post("/auth/logout")]
pub fn logout(cookie_jar: &rocket::http::CookieJar<'_>) -> rocket::response::Redirect {
    cookie_jar.remove_private("access_token");
    rocket::response::Redirect::to(rocket::uri!(super::render_index()))
}

pub fn routes() -> Vec<rocket::route::Route> {
    rocket::routes![self::handle_auth_callback, self::authorize, self::logout]
}

// fn test_rocket() -> Rocket<Build> {
//     rocket::Rocket::build().mount("/", routes())
// }
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[cfg(test)]
pub mod tests {
    #[rocket::async_test]
    pub async fn test_authorize() -> Result<(), Box<dyn std::error::Error>> {
        let rocket = super::super::rocket().await?;
        // let figment = rocket.figment();
        // let server_config = rocket.state::<&crate::ServerConfig>().unwrap();
        let client = rocket::local::asynchronous::Client::tracked(rocket).await?;

        // the authorize endpoint should always return html with a link to continue
        let request = client
            .get("/auth/authorize")
            .header(rocket::http::Accept::HTML);
        // let r2 = request.clone();
        // let server_config = r2.guard::<&State<crate::ServerConfig>>().await.unwrap();
        let response = request.dispatch().await;
        assert_eq!(
            response.status(),
            rocket::http::Status::Ok,
            "Response status should be OK"
        );
        assert_eq!(
            response.content_type().is_some(),
            true,
            "The response should have a content type"
        );
        assert_eq!(
            response.content_type().unwrap(),
            rocket::http::ContentType::HTML,
            "The authorize endpoint should return HTML"
        );
        let body = response.body();
        dbg!(&body);

        // let _index = body_string.find(&format!(r#"a href="https://{}"#,server_config.auth0_domain())).unwrap();

        // test that we can call the error page with an invalid state and successfully return HTML to test template rendering
        let response = client.get("/auth/callback?error=test_error&error_description=test_error_description&state=efa28d09-8ef9-4b2d-860d-76ec90537c0a").header(rocket::http::Accept::HTML).dispatch().await;
        assert_eq!(response.content_type().is_some(), true);
        assert_eq!(
            response.content_type().unwrap(),
            rocket::http::ContentType::HTML
        );
        assert_eq!(response.status(), rocket::http::Status::Unauthorized);

        // return HTML whenever something goes wrong that describes the error
        let response = client
            .get("/auth/callback?code=abcdefghijklmnop&state=efa28d09-8ef9-4b2d-860d-76ec90537c0a")
            .header(rocket::http::Accept::HTML)
            .dispatch()
            .await;
        assert_eq!(response.status(), rocket::http::Status::Unauthorized);
        assert_eq!(
            response
                .content_type()
                .is_some_and(|c| c == rocket::http::ContentType::HTML),
            true
        );

        Ok::<(), Box<dyn std::error::Error>>(())
    }
}
