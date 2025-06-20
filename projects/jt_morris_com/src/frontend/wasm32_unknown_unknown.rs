// #[macro_use]
// use super::window;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub(crate) static FAILED_TO_GET_WINDOW_OBJECT: &str =
    "Failed to get the window object. Are we in a browser environment?";
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub(crate) static FAILED_TO_GET_DOCUMENT_OBJECT: &str =
    "Failed to get the document object from the window object. Are we in a browser environment ";
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
macro_rules! window {
    () => {
        match web_sys::window() {
            Some(window) => window,
            None => {
                tracing::error!("{}", FAILED_TO_GET_WINDOW_OBJECT);
                return;
            }
        }
    };
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
macro_rules! document {
    () => {
        match window!().document() {
            Some(document) => document,
            None => {
                tracing::error!("{}", FAILED_TO_GET_DOCUMENT_OBJECT);
                return;
            }
        }
    };
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen::wasm_bindgen(start)]
#[no_mangle]
/// starts the client side application
/// WARNING: THIS FUNCTION WILL CAUSE PANICS IN NON BROWSER ENVIRONMENT
pub async fn start_frontend() -> Result<(), wasm_bindgen::JsValue> {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;

    // use the proto when on wasm32!
    use webauthn_rs_proto::{CreationChallengeResponse, RegisterPublicKeyCredential};
    use yew::AppHandle;

    use crate::frontend::net::FinishWebAuthnRegistrationBody;

    use super::{FrontendApp, FrontendAppProperties};
    use wasm_bindgen::prelude::*;
    use yew::Renderer;
    // set a panic handler
    std::panic::set_hook(Box::new(|panic_info: &std::panic::PanicInfo| {
        if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            let err_msg = format!("Web App Panicked! panic message {s}");
            let err_msg_jsvalue = JsValue::from_str(&err_msg);
            web_sys::console::error_1(&err_msg_jsvalue)
        } else {
            let err_msg_jsvalue = JsValue::from_str("Web App Panicked! no panic message available");
            web_sys::console::error_1(&err_msg_jsvalue)
        }
    }));
    // create a renderer that mounts at DocumentElement
    web_sys::console::log_1(&JsValue::from_str("starting web app"));
    let window = web_sys::window().unwrap();

    // we expect the server to insert an <input type="text" hidden=true id="initial-data" />
    // into the body of the webpage which is a base64-encoded messagepack data structure that holds the current route information

    let document_element = window.document().unwrap();
    let element = document_element
        .query_selector(r#"body > input[type="text"]#initial-data"#)
        .expect("Query selector error")
        .expect("Failed to find element");
    let html_element: web_sys::HtmlInputElement = element
        .dyn_into()
        .expect("Failed to cast element to input element for initial data");
    let value = html_element.value();

    use base64::Engine;
    let bytes = base64::prelude::BASE64_STANDARD
        .decode(value)
        .expect("Failed to base64 decode initial data");
    let initial_properties: FrontendAppProperties =
        rmp_serde::from_slice(&bytes).expect("failed to msgpack decode intial data");
    // yew expects an exact copy of what the client will render after the webpage loads, which DOES NOT include the intitial data element.
    html_element.remove();

    // mount to body element
    let body_element = document_element.body().unwrap();
    // let props = FrontendAppProperties {route};
    let _: AppHandle<FrontendApp> =
        Renderer::with_root_and_props(web_sys::Element::from(body_element), initial_properties)
            .hydrate();
    // let mut renderer: Renderer<FrontendApp> = Renderer::with_root(web_sys::Element::from(body_element));
    Ok(())
}

/// called after asking the server to create a challenge response. used to register the challenge with the browser
/// and ask the user to enter their pin code or use another passkey for this website
/// WARNING: THIS FUNCTION MAY PANIC IF NOT CALLED IN A BROWSER ENVIRONMENT.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub(crate) fn register_public_key<Callback>(
    passkey_state_id: uuid::Uuid,
    creation_challenge_response: CreationChallengeResponse,
    callback: Callback,
) where
    Callback: FnOnce(Result<(), JsValue>) -> () + 'static,
{
    macro_rules! ok_or_return_callback {
        ($r:expr) => {
            match $r {
                Ok(item) => item,
                Err(e) => {
                    return callback(Err(e));
                }
            }
        };
    }

    wasm_bindgen_futures::spawn_local(async move {
        let credential_creation_options =
            web_sys::CredentialCreationOptions::from(creation_challenge_response);
        let window = window!();
        let init_create_credentials_result = window
            .navigator()
            .credentials()
            .create_with_options(&credential_creation_options);
        let create_credentials_promise = ok_or_return_callback!(init_create_credentials_result);
        let public_key_credential_result = JsFuture::from(create_credentials_promise).await;
        let public_key_credential_value = ok_or_return_callback!(public_key_credential_result);
        let public_key_credential: web_sys::PublicKeyCredential =
            web_sys::PublicKeyCredential::from(public_key_credential_value);
        let register_public_key_credential: RegisterPublicKeyCredential =
            public_key_credential.into();
        let body = FinishWebAuthnRegistrationBody {
            passkey_state_id,
            public_key_request: register_public_key_credential,
        };
        let register_public_key_network_result = super::net::register_webauthn_public_key(body)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()));
        let register_public_key_response =
            ok_or_return_callback!(register_public_key_network_result);
        callback(Ok(register_public_key_response));
    });
}

/// asks the server to generate a credentials challenge response
// pub(crate) fn perform_ccr_request<Callback>(ccr_url: String, username: String, callback: Callback)
// where
//     Callback: FnOnce(Result<_, anyhow::Error>) -> (),
// {
//     //    perform the ccr request
//     wasm_bindgen_futures::spawn_local(async move {
//         let net_result = super::net::perform_ccr_request(ccr_url, username).await?;
//         let response = ok_or_return_callback!(net_result);
//         let output = ok_or_return_callback!(response);
//     })
// }

// pub(crate) async fn register_public_key(
//     body: FinishWebAuthnRegistrationBody,
//     setter: UseStateSetter<StartAuthenticationUIState>,
// ) {
//     request::<FinishWebAuthnRegistrationBody, (), _, _>(
//         "POST".to_string(),
//         "/authentication/finish-webauthn-registration".to_string(),
//         Some(body),
//         |request_result| async move {
//             match request_result {
//                 Ok(()) => setter.set(StartAuthenticationUIState::WaitingForInput { error: None }),
//                 Err(e) => {
//                     let error = e.as_string().unwrap_or_default();
//                     let error_message = format!("Failed to register public key: {error}");
//                     web_sys::console::error_2(&JsValue::from_str(&error_message), &e);
//                     setter.set(StartAuthenticationUIState::WaitingForInput {
//                         error: Some(error_message),
//                     })
//                 }
//             }
//         },
//     );
// }

/// attempts to set property window.document.title. does nothing when not in wasm32
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub fn set_window_title(title: &str) {
    let document = document!();
    document.set_title(&title);
}
