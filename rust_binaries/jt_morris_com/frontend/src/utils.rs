use std::{fmt::format, future::Future};

use js_sys::{ArrayBuffer, Uint8Array};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit};
use webauthn_rs_proto::RegisterPublicKeyCredential;
use yew::UseStateSetter;
/// makes a fetch request and serializes and deserializes the response with msgpack. reduces boilerplate
pub fn request<Body, Output, Callback, Fut>(
    method: String,
    url: String,
    body: Option<Body>,
    callback: Callback,
) where
    Body: Serialize + 'static,
    Output: for<'de> Deserialize<'de>,
    Callback: FnOnce(Result<Output, JsValue>) -> Fut + 'static,
    Fut: Future<Output = ()>,
{
    wasm_bindgen_futures::spawn_local(async move {
        let options = RequestInit::new();
        options.set_method(&method);
        options.set_mode(web_sys::RequestMode::SameOrigin);
        let body_bytes = match rmp_serde::to_vec_named(&body) {
            Ok(bytes) => bytes,
            Err(encoding_error) => {
                let error_message = format!("Rmp_serde encoding error: {encoding_error}");
                return callback(Err(JsValue::from_str(&error_message))).await;
            }
        };
        let body_array = unsafe { Uint8Array::view(&body_bytes) };
        let body_jsvalue: JsValue = match body_array.dyn_into() {
            Ok(jsvalue) => jsvalue,
            Err(_) => {
                const ERROR_MESSAGE: &str = "Failed to cast body Uint8Array to JsValue";
                return callback(Err(JsValue::from_str(&ERROR_MESSAGE))).await;
            }
        };
        options.set_body(&body_jsvalue);

        let request = match Request::new_with_str_and_init(&url, &options) {
            Ok(request) => request,
            Err(e) => {
                let error = e.as_string().unwrap_or_default();
                let error_message = format!("Failed to create an instance of Request {error}");
                return callback(Err(JsValue::from_str(&error_message))).await;
            }
        };
        match request.headers().set("content-type", "application/msgpack") {
            Ok(_) => {}
            Err(e) => {
                let error = e.as_string().unwrap_or_default();
                let error_message = format!("Failed to set request header {error}");
                return callback(Err(JsValue::from_str(&error_message))).await;
            }
        };
        match request.headers().set("accept", "application/msgpack") {
            Ok(_) => {}
            Err(e) => {
                let error = e.as_string().unwrap_or_default();
                let error_message = format!("Failed to set request header {error}");
                return callback(Err(JsValue::from_str(&error_message))).await;
            }
        };

        let window = match web_sys::window() {
            Some(w) => w,
            None => return callback(Err(JsValue::from_str(FAILED_TO_GET_WINDOW_OBJECT))).await,
        };
        let response_result = JsFuture::from(window.fetch_with_request(&request)).await;
        let response_value = match response_result {
            Ok(r) => r,
            Err(e) => {
                return callback(Err(e)).await;
            }
        };
        let response: web_sys::Response = match response_value.dyn_into() {
            Ok(r) => r,
            Err(e) => {
                return callback(Err(e)).await;
            }
        };
        let status = response.status();
        if !response.ok() {
            return callback(Err(JsValue::from_str(&format!(
                "Request failed with status code {status}"
            ))))
            .await;
        }
        let array_buffer_promise = match response.array_buffer() {
            Ok(buffer) => buffer,
            Err(e) => {
                return callback(Err(e)).await;
            }
        };
        let array_buffer_result = JsFuture::from(array_buffer_promise).await;
        let array_buffer_jsvalue = match array_buffer_result {
            Ok(buffer) => buffer,
            Err(e) => {
                return callback(Err(e)).await;
            }
        };
        let typed_array: Uint8Array = Uint8Array::new(&array_buffer_jsvalue);
        let bytes = typed_array.to_vec();
        let output_result: Result<Output, JsValue> =
            rmp_serde::from_slice(&bytes).map_err(|e| JsValue::from_str(&e.to_string()));
        callback(output_result).await;
    });
}

use crate::{
    types::{FinishWebAuthnRegistrationBody, StartWebAuthnRegistrationResponse},
    StartAuthenticationUIState,
};
pub(crate) static FAILED_TO_GET_WINDOW_OBJECT: &str =
    "Failed to get a reference to the window object";

pub(crate) fn perform_ccr_request(
    ccr_url: String,
    username: String,
    setter: UseStateSetter<StartAuthenticationUIState>,
) {
    let url = format!("{ccr_url}?username={username}");
    //    perform the ccr request
    request::<(), StartWebAuthnRegistrationResponse, _, _>(
        "POST".to_string(),
        url.to_string(),
        None,
        |r| async move {
            let response = match r {
                Ok(response) => response,
                Err(e) => {
                    let error = e.as_string().unwrap_or_default();
                    let error_message = format!("Error while performing ccr request: {error}");
                    web_sys::console::log_2(&JsValue::from_str(&error_message), &e);
                    setter.set(StartAuthenticationUIState::WaitingForInput {
                        error: Some(error_message),
                    });
                    return;
                }
            };
            // return early if the request is successful after updating the applicaton state. otherwise handle the errors
            static ERROR_PREFIX: &str = "CCR request failed";
            let error_message = match response {
                StartWebAuthnRegistrationResponse::Ok {
                    passkey_state_id,
                    ccr,
                } => {
                    setter.set(StartAuthenticationUIState::RegisteringChallenge {
                        passkey_state_id,
                        ccr,
                    });
                    return;
                }
                StartWebAuthnRegistrationResponse::DatabaseConnectionFailed(m) => {
                    format!("{ERROR_PREFIX}: database connection failed: {m}")
                }
                StartWebAuthnRegistrationResponse::MissingUsername => {
                    format!("{ERROR_PREFIX}: Missing request parameter username")
                }
                StartWebAuthnRegistrationResponse::UsernameLookupFailed(m) => {
                    format!("{ERROR_PREFIX}: username lookup failed: {m}")
                }
                StartWebAuthnRegistrationResponse::WebAuthnError(m) => {
                    format!("{ERROR_PREFIX}: Webauthn Error: {m}")
                }
            };
            web_sys::console::error_1(&JsValue::from_str(&error_message));
            setter.set(StartAuthenticationUIState::WaitingForInput {
                error: Some(error_message),
            });
        },
    )
}
pub(crate) async fn register_public_key(
    body: FinishWebAuthnRegistrationBody,
    setter: UseStateSetter<StartAuthenticationUIState>,
) {
    request::<FinishWebAuthnRegistrationBody, (), _, _>(
        "POST".to_string(),
        "/authentication/finish-webauthn-registration".to_string(),
        Some(body),
        |request_result| async move {
            match request_result {
                Ok(()) => setter.set(StartAuthenticationUIState::WaitingForInput { error: None }),
                Err(e) => {
                    let error = e.as_string().unwrap_or_default();
                    let error_message = format!("Failed to register public key: {error}");
                    web_sys::console::error_2(&JsValue::from_str(&error_message), &e);
                    setter.set(StartAuthenticationUIState::WaitingForInput {
                        error: Some(error_message),
                    })
                }
            }
        },
    );
}
