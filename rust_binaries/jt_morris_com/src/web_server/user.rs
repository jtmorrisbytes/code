// use rocket::time as time;
use bigdecimal::BigDecimal;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use proc_macros::IntoTemplate;
use std::default;

use super::authorization::AccessTokenRequestError;

// #[cfg(not(all(target_arch="wasm32",target_os="unknown")))]
// #[rocket::async_trait]
// impl<'r> rocket::request::FromRequest<'r> for User {
//     type Error = anyhow::Error;
//     async fn from_request(
//         _request: &'r rocket::request::Request<'_>,
//     ) -> rocket::request::Outcome<Self, Self::Error> {
//         todo!("From request for user")
//     }
// }

pub struct AccessTokenUser(pub(crate) super::db::user::User);
impl std::ops::Deref for AccessTokenUser {
    type Target = super::db::user::User;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(serde::Serialize, thiserror::Error, Debug)]
#[error("AcessTokenUserError: {0}")]
pub struct AccessTokenUserError(String);

impl std::convert::From<AccessTokenRequestError> for AccessTokenUserError {
    fn from(value: AccessTokenRequestError) -> Self {
        Self(value.to_string())
    }
}
impl std::convert::From<String> for AccessTokenUserError {
    fn from(s: String) -> Self {
        Self(s)
    }
}

pub struct AccessTokenUserId(pub uuid::Uuid);
impl std::ops::Deref for AccessTokenUserId {
    type Target = uuid::Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<uuid::Uuid> for AccessTokenUserId {
    fn as_ref(&self) -> &uuid::Uuid {
        &self.0
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl std::convert::From<AccessTokenUserError> for super::response::HtmlResponse {
    fn from(value: AccessTokenUserError) -> super::response::HtmlResponse {
        let context = super::response::UnauthorizedError {
            error: value.to_string(),
            return_uri: rocket::uri!(super::authorization::authorize(
                Option::<String>::None,
                Option::<String>::None
            ))
            .to_string(),
        };
        super::response::HtmlResponse::unauthorized(context)
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for AccessTokenUser {
    type Error = AccessTokenUserError;
    async fn from_request(
        request: &'r rocket::request::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        use super::authorization::AccessToken;
        // use super::user::User;
        use super::db::PrimaryDatabasePoolConnection;
        let access_token = match request.guard::<AccessToken>().await {
            rocket::request::Outcome::Success(a_t) => a_t,
            rocket::request::Outcome::Error((_status, error)) => {
                return rocket::request::Outcome::Error((
                    rocket::http::Status::Unauthorized,
                    error.into(),
                ))
            }
            rocket::request::Outcome::Forward(_) => unimplemented!(),
        };
        let mut connection = request
            .guard::<PrimaryDatabasePoolConnection>()
            .await
            .unwrap();
        let auth0_user_id = access_token.claims().sub.clone();
        let fetch_user_result =
            super::db::user::User::get_from_auth0_user_id(&mut connection, &auth0_user_id).await;
        match fetch_user_result {
            Ok(user) => rocket::request::Outcome::Success(Self(user)),
            // Ok(None) => rocket::request::Outcome::Error((rocket::http::Status::NotFound,format!("AccessTokenUser request guard: user with auth0_user_id {} not found",access_token.claims().sub).into())),
            Err(error) => rocket::request::Outcome::Error((rocket::http::Status::InternalServerError,error.context("Access token user request guard: while trying to get a user from thier access token").to_string().into())),
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for AccessTokenUserId {
    type Error = anyhow::Error;
    async fn from_request(
        request: &'r rocket::request::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        use super::db::PrimaryDatabasePoolConnection;
        // use super::user::User;
        // use rocket::tokio;
        let access_token = match request.guard::<super::authorization::AccessToken>().await {
            rocket::request::Outcome::Success(a_t) => a_t,
            rocket::request::Outcome::Error((_status, error)) => {
                return rocket::request::Outcome::Error((
                    rocket::http::Status::Unauthorized,
                    anyhow::Error::new(error),
                ))
            }
            rocket::request::Outcome::Forward(_) => unimplemented!(),
        };
        let mut connection = request
            .guard::<PrimaryDatabasePoolConnection>()
            .await
            .unwrap();
        match super::db::user::User::get_id_from_auth0_user_id(
            &mut connection,
            &access_token.claims().sub,
        )
        .await
        {
            Ok(user_id) => rocket::request::Outcome::Success(Self(user_id)),
            Err(error) => {
                return rocket::request::Outcome::Error((
                    rocket::http::Status::InternalServerError,
                    error.into(),
                ))
            }
        }
    }
}

pub struct UserProfile {}

// use crate::authentication::SCOPE_READ_OTHERS_DATA;
#[cfg_attr(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    derive(IntoTemplate)
)]
#[derive(serde::Serialize)]
pub struct UserSettings {
    dashboard_url: String,
    settings_url: String,
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::get("/user/settings.html", format = "text/html")]
pub async fn render_user_settings_template(
    #[allow(unused)] public_base_uri: super::RocketAbsoluteBaseUri,
    access_token_user: super::user::AccessTokenUser,
    // server_config: &rocket::State<crate::ServerConfig>,
    // connection: crate::PrimaryDatabaseConnection,
) -> super::response::HtmlResult {
    let _ = access_token_user;
    // let index_uri= rocket::uri!((*public_base_uri).to_owned(),crate::render_index());
    let dashboard_url = rocket::uri!(
        (*public_base_uri).to_owned(),
        render_user_dashboard_template()
    );
    let settings_url = rocket::uri!(
        (*public_base_uri).to_owned(),
        render_user_settings_template()
    );
    Ok(super::response::HtmlResponse::success_into_template(
        UserSettings {
            dashboard_url: dashboard_url.to_string(),
            settings_url: settings_url.to_string(),
        },
    ))
}

#[rocket::get("/user/transactions.html", format = "text/html")]
pub fn render_user_transactions_template(#[allow(unused)] access_token_user: AccessTokenUser) {}

#[derive(serde::Serialize, IntoTemplate)]
pub struct UserDashboardContext {
    pub settings_url: String,
    pub dashboard_url: String,
    pub transactions_upload_url: String,
    pub all_time_expenses: BigDecimal,
    pub all_time_income: BigDecimal,
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::get("/user/dashboard.html", format = "text/html")]
pub async fn render_user_dashboard_template(
    access_token_user: Result<super::user::AccessTokenUser, super::user::AccessTokenUserError>,
    // server_config: &rocket::State<crate::ServerConfig>,
    pool: Result<super::db::PrimaryDatabasePool, anyhow::Error>,
    public_base_uri: super::RocketAbsoluteBaseUri,
) -> super::response::HtmlResult {
    use super::response::HtmlResponse;
    let user = access_token_user?.0;
    let pool = pool?;
    // perform data fetching here

    // let all_time_expenses_result =
    // sqlx::query_scalar!("
    // SELECT SUM(total_cost)::text from transactions
    // JOIN users_transactions ON users_transactions.transactions_id=transactions.id AND users_transactions.user_id = $1
    // WHERE total_cost < 0
    // ",user.id).fetch_one(&mut **connection).await;
    // let all_time_expenses = crate::success_or_render_error_template(
    //     all_time_expenses_result,
    //     "attempting to calculate total expenses",
    //     &index_uri,
    // )?
    // .unwrap_or_default();
    let pool_clone = pool.inner();
    let user_id_clone = user.id.clone();
    let all_time_expenses = super::db::get_all_time_expenses_for_user(pool_clone, &user_id_clone)
        .await
        .unwrap_or_default()
        .unwrap_or_default();

    // let total_income = sqlx::query_scalar!("SELECT SUM(amount) from transactions where user_id = $1 and amount > 0 ").fetch_one(&mut **connection).await;
    // let all_time_income_result =
    // sqlx::query_scalar!("
    // SELECT SUM(total_cost)::text from transactions
    // JOIN users_transactions ON users_transactions.transactions_id=transactions.id AND users_transactions.user_id = $1
    // WHERE total_cost > 0
    // ",user.id).fetch_one(&mut **connection).await;
    // let all_time_income = crate::success_or_render_error_template(
    //     all_time_income_result,
    //     "attempting to calculate total income",
    //     &index_uri,
    // )?
    // .unwrap_or_default();

    let pool_clone = pool.inner();
    let all_time_income = super::db::get_all_time_income_for_user(pool_clone, &user.id)
        .await?
        .unwrap_or_default();
    Ok(HtmlResponse::success_into_template(UserDashboardContext {
        settings_url: rocket::uri!(
            (*public_base_uri).to_owned(),
            render_user_settings_template()
        )
        .to_string(),
        dashboard_url: rocket::uri!(
            (*public_base_uri).to_owned(),
            render_user_dashboard_template()
        )
        .to_string(),
        transactions_upload_url: rocket::uri!(
            (*public_base_uri).to_owned(),
            render_transactions_upload_template()
        )
        .to_string(),
        all_time_expenses,
        all_time_income,
    }))
}
#[rocket::get("/user/tools/paycheck-estimator.html")]
pub fn render_user_paycheck_estimator() {}

// create a function that allows a user to upload transactions from a file
// I want to allow multipart file uploads of an unlimited amount and size.
// #[derive(FromForm,Debug)]
// pub struct CsvFileUpload {
//     name:String,
//     file:String
// }
// #[derive(Debug)]
// pub struct Multipart{}
// #[rocket::async_trait]
// impl<'r> rocket::data::FromData<'r> for Multipart {
//     type Error = anyhow::Error;
//     fn from_data(request: &'r rocket::request::Request<'_>,data: rocket::Data<'r>) -> rocket::data::Outcome<'r,Self,Self::Error> {

//         rocket::data::Outcome::Success(Self{})
//     }
// }
#[derive(serde::Serialize, default::Default, IntoTemplate)]
pub struct UserTransactionsUploadTemplateContext {
    database_errors: Vec<String>,
    parse_errors: Vec<String>,
    successfully_uploaded: usize,
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::get("/user/transactions/upload", format = "text/html")]
pub async fn render_transactions_upload_template() -> super::response::HtmlResult {
    let context = UserTransactionsUploadTemplateContext::default();
    Ok(super::response::HtmlResponse::success_into_template(
        context,
    ))
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::post(
    "/user/transactions/upload",
    data = "<data>",
    format = "multipart/form-data"
)]
pub async fn upload_user_transactions_from_csv(
    user_id_result: Result<AccessTokenUserId, anyhow::Error>,
    pool_result: Result<super::db::PrimaryDatabasePool, anyhow::Error>,
    data: rocket::form::Form<std::collections::HashMap<String, rocket::fs::TempFile<'_>>>,
) -> Result<super::response::HtmlResponse, super::response::HtmlResponse> {
    let user_id = user_id_result?;
    let pool = pool_result?;
    let mut parse_errors: Vec<String> = Vec::new();
    let mut database_errors: Vec<String> = Vec::new();
    let mut successfully_uploaded = 0;
    tracing::info!("{} files", data.len());
    for (key, temp_file) in data.iter() {
        println!("{key}");
        let path = temp_file.path().ok_or("File path was none")?.to_owned();
        let pool_clone = pool.inner();
        let (successfully_uploaded_, errors) =
            super::db::transactions::import_bank_of_america_file_for_user(
                pool_clone, &user_id, path,
            )
            .await?;
        parse_errors.append(
            errors
                .parse_errors
                .iter()
                .map(|error| error.to_string())
                .collect::<Vec<String>>()
                .as_mut(),
        );
        database_errors.append(
            errors
                .database_errors
                .iter()
                .map(|error| error.to_string())
                .collect::<Vec<String>>()
                .as_mut(),
        );
        successfully_uploaded = successfully_uploaded + successfully_uploaded_;
        // let parse_errors: Vec<String> = errors.parse_errors.iter().map(|e| e.to_string()).collect();
        // tracing::info!("database errors:\n{}", database_error_string);
        // tracing::info!("parse errors:\n{}",errors.parse_errors)
    }
    let context = UserTransactionsUploadTemplateContext {
        database_errors,
        parse_errors,
        successfully_uploaded,
    };
    Ok(super::response::HtmlResponse::success_into_template(
        context,
    ))
    // dbg!(&data);
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(rocket::Responder)]
pub enum GetUsernameResponse<T, E> {
    Ok(rocket::serde::msgpack::MsgPack<T>),
    #[response(status = 500)]
    Err(rocket::serde::msgpack::MsgPack<E>),
    #[response(status = 400)]
    NotFound(()),
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl<T, E> std::convert::From<Result<T, E>> for GetUsernameResponse<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(t) => Self::Ok(rocket::serde::msgpack::MsgPack(t)),
            Err(e) => Self::Err(rocket::serde::msgpack::MsgPack(e)),
        }
    }
}
