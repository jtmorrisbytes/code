// HTML RESPONSE
// pub type HtmlResult<T> = Result<HtmlResponse<T>>

pub use proc_macros::IntoTemplate;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(rocket::Responder)]
pub enum HtmlResponse {
    #[response(status = 200)]
    Success(rocket_dyn_templates::Template),
    #[response(status = 500)]
    InternalServerError(rocket_dyn_templates::Template),
    #[response(status = 401)]
    Unauthorized(rocket_dyn_templates::Template),
    #[response(status = 400)]
    BadRequest(rocket_dyn_templates::Template),
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl HtmlResponse {
    pub fn success_into_template<T: serde::Serialize + IntoTemplate>(val: T) -> Self {
        Self::Success(val.into_template())
    }
    pub fn success(template: rocket_dyn_templates::Template) -> Self {
        Self::Success(template)
    }
    pub fn unauthorized(context: UnauthorizedError) -> Self {
        Self::Unauthorized(context.into_template())
    }
    pub fn bad_request(context: BadRequestError) -> Self {
        Self::BadRequest(context.into_template())
    }
    pub fn internal_server_error(context: InternalServerError) -> Self {
        Self::InternalServerError(context.into_template())
    }
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]

pub type HtmlResult = Result<HtmlResponse, HtmlResponse>;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(serde::Serialize, IntoTemplate)]
pub struct InternalServerError {
    // #[response(ignore)]
    pub error: String,
    // #[response(ignore)]
    pub return_uri: String,
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]

impl std::default::Default for InternalServerError {
    fn default() -> Self {
        Self {
            error: "An unhandled server error occurred".to_string(),
            return_uri: "/".to_string(),
        }
    }
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(serde::Serialize, IntoTemplate)]
pub struct UnauthorizedError {
    pub error: String,
    pub return_uri: String,
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]

impl std::default::Default for UnauthorizedError {
    fn default() -> Self {
        Self {
            error: "You were not authorized to perform an action".to_string(),
            return_uri: "/".to_string(),
        }
    }
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub trait IntoTemplate {
    fn into_template(self) -> rocket_dyn_templates::Template;
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[derive(serde::Serialize, IntoTemplate)]
pub struct BadRequestError {
    pub error: String,
    pub return_uri: String,
}
impl std::convert::From<anyhow::Error> for HtmlResponse {
    fn from(error: anyhow::Error) -> Self {
        Self::internal_server_error(InternalServerError {
            error: error.to_string(),
            return_uri: rocket::uri!(super::render_index()).to_string(),
        })
    }
}
impl std::convert::From<auth0::Error> for HtmlResponse {
    fn from(error: auth0::Error) -> Self {
        Self::unauthorized(UnauthorizedError {
            error: error.to_string(),
            return_uri: rocket::uri!(super::authorization::authorize(
                Option::<String>::None,
                Option::<String>::None
            ))
            .to_string(),
        })
    }
}
impl std::convert::From<&str> for HtmlResponse {
    fn from(value: &str) -> Self {
        Self::internal_server_error(InternalServerError {
            error: value.to_string(),
            return_uri: rocket::uri!(super::render_index()).to_string(),
        })
    }
}
impl std::convert::From<String> for HtmlResponse {
    fn from(value: String) -> Self {
        Self::internal_server_error(InternalServerError {
            error: value,
            return_uri: rocket::uri!(super::render_index()).to_string(),
        })
    }
}
// impl std::convert::From<crate::user::AccessTokenUserError> for HtmlResponse {
//     fn from(error: crate::user::AccessTokenUserError) -> Self {
//         Self::internal_server_error(InternalServerErrorContext{error:error.to_string(),return_uri:rocket::uri!(crate::render_index()).to_string()})
//     }
// }
