use serde::Deserialize;
use std::{collections::HashMap, fmt::Debug};

#[deny(unused_imports, unused_import_braces)]
pub mod management;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(any(feature = "debug", debug_assertions), derive(Debug))]
pub struct AccessTokenResponse {
    access_token: String,
    expires_in: i64,
    token_type: String,
}
// #[cfg_attr(any(feature = "debug", debug_assertions), derive(Debug))]
#[derive(PartialEq, Debug)]
pub struct AccessToken<Claims: for<'de> serde::Deserialize<'de> + PartialEq + Debug> {
    access_token: String,
    claims: Claims,
    // expires: time::OffsetDateTime,
}

impl<Claims: for<'de> serde::Deserialize<'de> + PartialEq + Debug> AccessToken<Claims> {
    pub fn claims(&self) -> &Claims {
        &self.claims
    }
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
    // pub fn expires(&self) -> &time::OffsetDateTime {
    //     &self.expires
    // }
}
pub async fn get_jwks(base_url: &url::Url) -> Result<jsonwebtoken::jwk::JwkSet, self::Error> {
    let url = base_url.join(".well-known/jwks.json")?;
    Ok(reqwest::get(url).await?.json().await?)
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum Error {
    #[error(
        "The caller should re-attempt this operation as the client is using exponential backoff"
    )]
    RetryWithBackoff { delay: u64 },
    #[error("Authentication Error: {0}")]
    AuthenticationError(AuthenticationError),
    #[error("Json Parse Error: {0}")]
    SerdeJsonParseError(String),
    #[error("Time library error: {0}")]
    TimeError(String),
    #[error("Url parse error: {0}")]
    UrlParseError(String),
    #[error("Jsonwebtoken library error {0}")]
    JsonwebtokenError(String),
    #[error("Reqwest library error")]
    ReqwestError(String),
    #[error("General Errror: {0}")]
    Other(String),
}
impl std::convert::From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value.to_string())
    }
}
impl std::convert::From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::JsonwebtokenError(value.to_string())
    }
}
impl std::convert::From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParseError(value.to_string())
    }
}
impl std::convert::From<time::error::ComponentRange> for Error {
    fn from(value: time::error::ComponentRange) -> Self {
        Self::TimeError(value.to_string())
    }
}
impl std::convert::From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJsonParseError(value.to_string())
    }
}

impl std::convert::Into<Error> for AuthenticationError {
    fn into(self) -> Error {
        Error::AuthenticationError(self)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug, thiserror::Error)]
#[error("Authentication Error: {error}. {error_message}")]
pub struct AuthenticationError {
    error: String,
    error_message: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(any(feature = "debug", debug_assertions), derive(Debug))]
pub struct ClientCredentialsOptions {
    pub audience: String,
    pub scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<AccessType>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(any(feature = "debug", debug_assertions), derive(Debug))]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    ClientCredentials,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(any(feature = "debug", debug_assertions), derive(Debug))]
pub enum AccessType {
    Offline,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct GetAccessTokenUsingClientCredentialsBody {
    pub client_id: String,
    pub client_secret: String,
    pub audience: String,
    pub grant_type: GrantType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<AccessType>,
    pub scope: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum JwtAlgorythm {
    RS256,
    HS256,
}
impl std::convert::Into<jsonwebtoken::Algorithm> for JwtAlgorythm {
    fn into(self) -> jsonwebtoken::Algorithm {
        match self {
            Self::HS256 => jsonwebtoken::Algorithm::HS256,
            Self::RS256 => jsonwebtoken::Algorithm::RS256,
        }
    }
}
impl std::default::Default for JwtAlgorythm {
    fn default() -> Self {
        Self::RS256
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(any(feature = "debug", debug_assertions), derive(Debug))]
pub struct JwtClaims {
    sub: String,
    #[serde(flatten)]
    additional_claims: HashMap<String, serde_json::Value>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct AuthenticationClientOptions {
    pub auth0_domain: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub jwks: Option<jsonwebtoken::jwk::JwkSet>,
    pub jwt_algorythm: Option<JwtAlgorythm>,
}

pub enum ResponseType {
    Code,
    CodeToken,
    CodeIdToken,
}

pub struct AuthenticationClient {
    client: reqwest::Client,
    base_url: url::Url,
    client_id: String,
    client_secret: Option<String>,
    jwks: jsonwebtoken::jwk::JwkSet,
    jwt_algorythm: jsonwebtoken::Algorithm,
    // delay: std::sync::atomic::AtomicU64,
}
pub struct StartLoginWithRedirectOptions<'a> {
    pub scope: &'a str,
    pub audience: &'a str,
    pub redirect_uri: &'a str,
    pub state: &'a str,
    pub connection: &'a str,
    pub additional_parameters: std::collections::HashMap<&'a str, &'a str>,
}

impl AuthenticationClient {
    pub async fn try_initialize(options: AuthenticationClientOptions) -> Result<Self, self::Error> {
        let AuthenticationClientOptions {
            auth0_domain,
            client_id,
            client_secret,
            jwks,
            jwt_algorythm,
        } = options;
        #[allow(unused_mut)]
        let mut client_builder = reqwest::Client::builder();

        // this option is not supported in wasm32
        #[cfg(not(target_arch = "wasm32"))]
        {
            client_builder = client_builder.https_only(true);
        }
        // only enable the three following options when enabled via compile flag
        #[cfg(feature = "deflate")]
        {
            client_builder = client_builder.deflate(true);
        }

        #[cfg(feature = "gzip")]
        {
            client_builder = client_builder.gzip(true);
        }
        #[cfg(feature = "brotli")]
        {
            client_builder = client_builder.brotli(true)
        }
        let client = client_builder.build()?;
        let base_url = url::Url::parse(&format!("https://{auth0_domain}"))?;
        let jwks = {
            if jwks.is_none() {
                get_jwks(&base_url).await?
            } else {
                jwks.unwrap()
            }
        };
        let jwt_algorythm: jsonwebtoken::Algorithm = jwt_algorythm.unwrap_or_default().into();

        Ok(Self {
            client,
            client_id,
            client_secret,
            base_url,
            jwks,
            jwt_algorythm,
        })
    }
    pub fn verify_token<Claims: for<'de> serde::Deserialize<'de> + PartialEq + Debug>(
        &self,
        token: &str,
        audience: &str,
    ) -> Result<AccessToken<Claims>, self::Error> {
        // verify the access token before returning it back to the caller.
        let token_header: jsonwebtoken::Header = jsonwebtoken::decode_header(token)?;
        let token_alg = token_header.alg;
        let token_kid = token_header.kid;
        if token_alg != self.jwt_algorythm {
            return Err(self::Error::Other("Cannot verify the JWT returned from the authorization server because the provided 'alg' value does not match the conifgured 'alg' value in the options.".to_string()));
        }
        if token_kid.is_none() {
            return Err(self::Error::Other("Cannot verify the JWT returned from the authorization server because the 'kid' claim was not provided in the access token header".to_string()));
        }
        let token_kid = token_kid.unwrap();
        let jwk = self.jwks.find(&token_kid).ok_or_else(|| self::Error::Other("Cannot verify the jwt because no jwks provided by the authorization server have a 'kid' claim that also matches 'kid' claim provided in the header of the jwt also provided by the authorization server".to_string()))?;
        let mut validation = jsonwebtoken::Validation::new(self.jwt_algorythm);
        validation.set_audience(&[audience]);
        let decoding_key = jsonwebtoken::DecodingKey::from_jwk(jwk)?;
        let token_data: jsonwebtoken::TokenData<Claims> =
            jsonwebtoken::decode(token, &decoding_key, &validation)?;

        let token: AccessToken<Claims> = AccessToken {
            access_token: token.to_string(),
            claims: token_data.claims,
            // expires: time::OffsetDateTime::from_unix_timestamp(response.expires_in)?,
        };
        Ok(token)
    }
    pub async fn delay(delay: u64) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            tokio::time::sleep(std::time::Duration::from_millis(delay)).await
        }
        #[cfg(target_arch = "wasm32")]
        {
            std::thread::sleep(std::time::Duration::from_millis(delay))
        }
    }
    pub fn exponential_backoff(delay: u64) -> u64 {
        delay.checked_mul(2).unwrap_or(u64::MAX)
    }
    pub async fn get_access_token_using_client_credentials_async<
        Claims: for<'de> serde::Deserialize<'de> + PartialEq + Debug,
    >(
        &self,
        options: &ClientCredentialsOptions,
        backoff_millis_u64: u64,
    ) -> Result<AccessToken<Claims>, self::Error> {
        Self::delay(backoff_millis_u64).await;

        if self.client_secret.is_none() {
            return Err(self::Error::Other(
                "Cannot use the client credentials flow when client secret is None".to_string(),
            ));
        }
        let body: GetAccessTokenUsingClientCredentialsBody =
            GetAccessTokenUsingClientCredentialsBody {
                client_id: self.client_id.clone(),
                client_secret: self.client_secret.clone().unwrap(),
                access_type: options.access_type.clone(),
                audience: options.audience.clone(),
                grant_type: GrantType::ClientCredentials,
                scope: options.scope.clone(),
            };
        let response = self
            .client
            .post(self.base_url.join("oauth/token")?)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("error while decoding response body after POST oauth/token {e}");
                e
            })?;
        let access_token = match response.status() {
            reqwest::StatusCode::BAD_REQUEST
            | reqwest::StatusCode::UNAUTHORIZED
            | reqwest::StatusCode::FORBIDDEN => Err(self::Error::AuthenticationError(
                response
                    .json::<self::AuthenticationError>()
                    .await
                    .map_err(|e| {
                        tracing::error!("Error while decoding error response body {e}");
                        e
                    })?,
            )),
            reqwest::StatusCode::TOO_MANY_REQUESTS => Err(self::Error::RetryWithBackoff {
                delay: Self::exponential_backoff(backoff_millis_u64),
            }),
            reqwest::StatusCode::OK => {
                let response = response.json::<AccessTokenResponse>().await?;
                let access_token: AccessToken<Claims> =
                    self.verify_token(&response.access_token, options.audience.as_str())?;
                Ok(access_token)
            }
            _ => unimplemented!("Recieved unexpected status code {}", response.status()),
        }?;
        Ok(access_token)
    }
    pub fn start_login_with_redirect(
        &self,
        options: &StartLoginWithRedirectOptions,
        _backoff_millis_u64: u64,
    ) -> Result<url::Url, anyhow::Error> {
        // let redirect_uri_string = redirect_uri.to_string();
        // let reirect_uri_encoded = urlencoding::encode(&redirect_uri_string);
        let StartLoginWithRedirectOptions {
            redirect_uri,
            additional_parameters,
            audience,
            scope,
            state,
            connection,
        } = options;

        let response_type = self
            .client_secret
            .as_ref()
            .map(|_| "code")
            .unwrap_or("token");
        let mut url = self.base_url.join("/authorize")?;
        let mut url = url
            .query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &redirect_uri)
            .append_pair("state", state)
            .append_pair("connection", connection)
            .append_pair("audience", audience)
            .append_pair("response_type", response_type)
            .append_pair("scope", scope)
            .finish()
            .to_owned();

        let mut pairs = url.query_pairs_mut();
        for (key, value) in additional_parameters.iter() {
            pairs.append_pair(key, value);
        }
        Ok(pairs.finish().to_owned())
    }
    pub async fn exchange_code_for_token<Claims: for<'de> Deserialize<'de> + PartialEq + Debug>(
        &self,
        scope: &str,
        redirect_uri: &str,
        state: &str,
        code: &str,
        audience: &str,
        backoff_millis_u64: u64,
    ) -> Result<AccessToken<Claims>, self::Error> {
        let client_secret = self.client_secret.as_ref().ok_or_else(|| self::Error::Other("Exchanging an authorization code for an access token is not available when client_secret is None".to_string()))?;
        let url = self.base_url.join("oauth/token")?;
        let mut body: HashMap<&'static str, &str> = HashMap::new();
        body.insert("client_id", &self.client_id);
        body.insert("client_secret", &client_secret);
        body.insert("grant_type", "authorization_code");
        body.insert("code", &code);
        body.insert("redirect_uri", redirect_uri);
        body.insert("audience", &audience);
        body.insert("scope", &scope);
        body.insert("state", state);
        let response = self.client.post(url).json(&body).send().await?;
        match response.status() {
            reqwest::StatusCode::BAD_REQUEST
            | reqwest::StatusCode::UNAUTHORIZED
            | reqwest::StatusCode::FORBIDDEN => Err(self::Error::AuthenticationError(
                response.json::<self::AuthenticationError>().await?,
            )),
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                return Err(self::Error::RetryWithBackoff {
                    delay: Self::exponential_backoff(backoff_millis_u64),
                })
            }
            reqwest::StatusCode::OK => {
                let response = response.json::<AccessTokenResponse>().await?;
                let access_token: AccessToken<Claims> =
                    self.verify_token(&response.access_token, audience)?;
                Ok(access_token)
            }
            _ => unimplemented!("Recieved unexpected status code {}", response.status()),
        }
    }
    pub async fn get_user_info<
        UserInfo: for<'de> serde::Deserialize<'de>,
        Claims: for<'de> serde::Deserialize<'de> + PartialEq + Debug,
    >(
        &self,
        access_token: &AccessToken<Claims>,
        backoff_millis_u64: u64,
    ) -> Result<UserInfo, self::Error> {
        let url = self.base_url.join("userinfo")?;
        let response = self
            .client
            .get(url)
            .bearer_auth(access_token.access_token())
            .send()
            .await?;
        match response.status() {
            reqwest::StatusCode::BAD_REQUEST
            | reqwest::StatusCode::UNAUTHORIZED
            | reqwest::StatusCode::FORBIDDEN => {
                let text = response.text().await?;
                let parse_result = serde_json::from_str::<self::AuthenticationError>(&text).map(|e| e.into()).map_err(|e|{
                    let serde_error = e.to_string();
                    let error_message = format!("Failed to serialize auth0::AuthenticationError while attempting to handle an error response. Serde Error '{serde_error}'. Respone body follows:\n\n{text}");
                    tracing::error!("{error_message}");
                    self::Error::SerdeJsonParseError(error_message)
                });
                Err(parse_result?)
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                return Err(self::Error::RetryWithBackoff {
                    delay: backoff_millis_u64,
                })
            }
            reqwest::StatusCode::OK => {
                let user_info = response.json::<UserInfo>().await?;

                Ok(user_info)
            }
            _ => unimplemented!("Recieved unexpected status code {}", response.status()),
        }
    }
}
