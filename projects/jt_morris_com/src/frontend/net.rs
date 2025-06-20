#![allow(warnings)]

use serde::{Deserialize, Serialize};
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use webauthn_rs::prelude::{CreationChallengeResponse, RegisterPublicKeyCredential};
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use webauthn_rs_proto::{CreationChallengeResponse, RegisterPublicKeyCredential};

//start webauthn registration
#[allow(warnings)]
macro_rules! ok_or_return {
    ($r:expr) => {
        match $r {
            Ok(response) => response,
            Err(reqwest_error) => {
                tracing::error!("Request Error:  {reqwest_error}");
                return;
            }
        }
    };
}

#[derive(Serialize, Deserialize)]
pub struct StartWebAuthnRegistrationResponse {
    pub passkey_state_id: uuid::Uuid,
    pub ccr: CreationChallengeResponse,
}
#[derive(Serialize, Deserialize, Debug, thiserror::Error)]
#[error("Start webautn registration failed {0}")]
pub enum StartWebAuthnRegistrationError {
    #[error("Missing username in request parameters")]
    MissingUsername,
    #[error("Database connection failed {0}")]
    DatabaseConnectionFailed(String),
    #[error("username lookup failed {0}")]
    UsernameLookupFailed(String),
    #[error("WebAuthnError {0}")]
    WebAuthnError(String),
}
/// returned from perform_ccr_request
pub type StartWebAuthnRegistrationResponseBody =
    Result<StartWebAuthnRegistrationResponse, StartWebAuthnRegistrationError>;

#[derive(Serialize, Deserialize)]
pub struct FinishWebAuthnRegistrationBody {
    pub passkey_state_id: uuid::Uuid,
    pub public_key_request: RegisterPublicKeyCredential,
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum FinishWebAuthnRegistrationResponseErrorKind {
    #[error("Invalid Passkey State")]
    InvalidPasskeyState,
    #[error("WebAuthnError: {0}")]
    WebAuthnError(String),
}
#[allow(unused)]
pub type FinishWebAuthnRegistrationResponseBody =
    Result<(), FinishWebAuthnRegistrationResponseErrorKind>;

pub async fn perform_msgpack_request<Body, Output>(
    method: reqwest::Method,
    url: &str,
    body: &Body,
) -> Result<Output, anyhow::Error>
where
    Body: Serialize,
    Output: for<'de> Deserialize<'de>,
{
    #[allow(unused_mut)]
    let mut client = reqwest::Client::builder();
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        client = client.https_only(true);
    }
    let client = client.build()?;
    let bytes = rmp_serde::to_vec_named(&body)?;
    let response = client
        .request(method, url)
        .body(bytes)
        .header("content-type", "application/msgpack")
        .header("accept", "application/msgpack")
        .fetch_mode_no_cors()
        .send()
        .await?
        .error_for_status()?;
    let bytes = response.bytes().await?;
    let output: Output = rmp_serde::from_slice(&bytes)?;
    Ok(output)
}
pub async fn generate_credentials_challenge_response(
    _ccr_url: &str,
    username: &str,
) -> Result<StartWebAuthnRegistrationResponseBody, anyhow::Error> {
    let url = format!("/authentication/start-webauthn-registration?username={username}");
    perform_msgpack_request(reqwest::Method::POST, &url, &()).await
}
pub async fn register_webauthn_public_key(
    body: FinishWebAuthnRegistrationBody,
) -> Result<(), anyhow::Error> {
    const URL: &str = "/authentication/finish-webauthn-registration";
    perform_msgpack_request(reqwest::Method::POST, URL, &body).await
}
