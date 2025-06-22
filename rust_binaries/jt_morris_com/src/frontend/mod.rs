// #[cfg(all(target_arch="wasm32",target_os="unknown"))]
pub(crate) mod wasm32_unknown_unknown;
// #[cfg(not(all(target_arch="wasm32",target_os="unknown")))]
pub(crate) mod net;
pub(crate) mod not_wasm32_unknown_unknown;
pub(crate) mod types;

pub use rmp_serde;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use types::StartAuthenticationUIState;
use yew::prelude::*;

#[derive(PartialEq, Properties, Default)]
pub struct HtmlDocumentProperties {
    pub children: Html,
    pub title: String,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub enum Route {
    StartAuthentication {
        // title:String,
        ccr_url: String,
        username: Option<String>,
    },
}
impl Route {
    /// determines the title of the webpage depending on the route used
    pub fn title(&self) -> &'static str {
        match self {
            &Self::StartAuthentication {
                ccr_url: _,
                username: _,
            } => "Start Authentication",
        }
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct StartAuthenticationPageProperties {
    pub ccr_url: Arc<String>,
    pub username: Arc<Option<String>>,
}
impl std::default::Default for StartAuthenticationPageProperties {
    fn default() -> Self {
        Self {
            ccr_url: Arc::new("./".to_string()),
            username: Arc::new(None),
        }
    }
}
/// starts the ccr request when the form is submitted

#[yew::function_component(StartAuthenticationPage)]
fn start_authentication(properties: &StartAuthenticationPageProperties) -> yew::Html {
    // let username_error = use_state(|| properties.username.clone());
    // let request_error = use_state(|| Arc::new(String::new()));
    let ui_state = use_state(|| StartAuthenticationUIState::WaitingForInput { error: None });
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    {
        let ui_state_setter = ui_state.setter();
        let ui_state_setter2 = ui_state.setter();
    }
    // let request_error_setter = request_error.setter();
    // perform the ccr request in an effect
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    {
        use_effect_with(ui_state.clone(), |state| {
            let state = (**state).to_owned();
            match state {
                // performs the ccr request when the ui transitions into this state
                StartAuthenticationUIState::PerformingCCR { username, ccr_url } => {
                    wasm_bindgen_futures::spawn_local(async move {
                        match net::generate_credentials_challenge_response(&ccr_url, &username)
                            .await
                        {
                            Ok(Ok(net::StartWebAuthnRegistrationResponse {
                                passkey_state_id,
                                ccr,
                            })) => {
                                // update state
                                ui_state_setter2.set(
                                    StartAuthenticationUIState::RegisteringChallenge {
                                        passkey_state_id,
                                        ccr,
                                    },
                                )
                            }
                            Ok(Err(registration_error)) => {
                                // log error, update state
                                ui_state_setter2.set(StartAuthenticationUIState::WaitingForInput {
                                    error: Some(registration_error.to_string()),
                                })
                            }
                            Err(reqwest_error) => {
                                // log error, update state
                                ui_state_setter2.set(StartAuthenticationUIState::WaitingForInput {
                                    error: Some(reqwest_error.to_string()),
                                })
                            }
                        }
                    })
                }
                //
                StartAuthenticationUIState::RegisteringChallenge {
                    passkey_state_id,
                    ccr,
                } => {
                    wasm32_unknown_unknown::register_public_key(passkey_state_id, ccr, |_result| {
                        // finalize authorization?
                    });
                }
                _ => {
                    return;
                }
            };
            // let setter = state.setter();
        });
    }
    let html = match *ui_state {
        StartAuthenticationUIState::WaitingForInput { ref error } => {
            let error_html = {
                if let Some(msg) = error {
                    html! {<p>{msg}</p>}
                } else {
                    html! {<p>{""}</p>}
                }
            };
            #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
            let obsubmit = move |event: SubmitEvent| {
                event.prevent_default();
                use wasm_bindgen::JsCast;
                let form_extract_result: Result<_, String> = (|| {
                    let target: web_sys::EventTarget = match event.target() {
                        Some(target) => target,
                        None => {
                            return Err(
                                "Failed to get the target of the submit event for this form"
                                    .to_string(),
                            );
                        }
                    };
                    let form: web_sys::HtmlFormElement = match target.dyn_into() {
                        Ok(f) => f,
                        Err(_) => {
                            return Err("Failed to cast the target element into a HTMLFormElement"
                                .to_string());
                        }
                    };
                    let form_data: web_sys::FormData = match web_sys::FormData::new_with_form(&form)
                    {
                        Ok(data) => data,
                        Err(e) => {
                            let error = e.as_string().unwrap_or_default();
                            let error_message = format!(
                                    "Failed to create an instance of FormData from the HTMLFormElement because: {error}"
                                );
                            // request_error.set(Arc::new(f);
                            return Err(error_message);
                        }
                    };
                    let ccr_url = form.action();
                    let username = form_data.get("username").as_string().unwrap();
                    Ok((username, ccr_url))
                })();
                match form_extract_result {
                    Ok((username, ccr_url)) => ui_state_setter
                        .set(StartAuthenticationUIState::PerformingCCR { username, ccr_url }),
                    Err(e) => ui_state_setter
                        .set(StartAuthenticationUIState::WaitingForInput { error: Some(e) }),
                };
            };
            #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
            let onsubmit = |_| ();
            html! {
                <div>
                    <h1>{"Welcome to the website"}</h1>
                    <p>{"To continue, please enter your username"}</p>
                    <form method={"POST"} action={properties.ccr_url.to_string()} onsubmit={onsubmit}>
                        <input type={"text"} name={"username"} minlength={1} required=true/>
                        <button type={"submit"}>{"Start Registration"}</button>
                    </form>
                    {error_html}
                </div>
            }
        }
        StartAuthenticationUIState::PerformingCCR {
            ref username,
            ccr_url: _,
        } => {
            html! {
                <div>
                    <p>{format!("Performing CCR for {username} Please Wait...",)}</p>
                </div>
            }
        }
        StartAuthenticationUIState::RegisteringChallenge {
            passkey_state_id: _,
            ccr: _,
        } => {
            html! {
                <div>
                    <p>{"Registering Challenge with browser"}</p>
                </div>
            }
        }
    };
    html
}

#[derive(Serialize, Deserialize, Properties, PartialEq)]
pub struct FrontendAppProperties {
    pub title: String,
    pub route: Route,
}

#[yew::function_component(FrontendApp)]
pub fn frontend_app(properties: &FrontendAppProperties) -> yew::Html {
    // create an effect which updates the title of the webpage whenever properties.title changes
    // setting window.title is only available on wasm32. we may need to cfg() this if it panics
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    {
        let title = properties.title.clone();
        use_effect_with((), move |_| {
            // LOL rust even supports IIFE like closures.
            wasm32_unknown_unknown::set_window_title(&title)
        });
    }

    match &properties.route {
        Route::StartAuthentication { ccr_url, username } => {
            html!(<StartAuthenticationPage ccr_url={Arc::new(ccr_url.clone())} username={Arc::new(username.clone())} />)
        }
    }
}
