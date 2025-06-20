pub mod scopes;
#[allow(unused)]
pub struct Client {
    client: reqwest::Client,
    base_url: url::Url,
    access_token: String,
}

impl Client {
    async fn _get_jwks(base_url: &url::Url) -> Result<jsonwebtoken::jwk::JwkSet, anyhow::Error> {
        let url = base_url.join(".well-known/jwks.json")?;
        Ok(reqwest::get(url).await?.json().await?)
    }

    pub async fn try_new(
        base_url: impl AsRef<str>,
        access_token: impl AsRef<str>,
        // jwks: jsonwebtoken::jwk::JwkSet,
    ) -> Result<Self, anyhow::Error> {
        let base_url: url::Url = url::Url::parse(base_url.as_ref())?;

        let jsonwebtoken::Header { kid, .. } = jsonwebtoken::decode_header(access_token.as_ref())?;
        if kid.is_none() {
            return Err(anyhow::Error::msg(
                "cannot verify access token because the 'kid' claim was not provided",
            ));
        }
        let _jwk: jsonwebtoken::jwk::Jwk = {
            let kid = kid.unwrap();
            let jwks = Self::_get_jwks(&base_url).await?;
            let jwk = jwks.find(&kid);
            if jwk.is_some() {
                Ok(jwk.unwrap().to_owned())
            } else {
                let jwks = Self::_get_jwks(&base_url).await?;
                jwks.find(&kid).map(|jwk| jwk.to_owned()).ok_or_else(|| anyhow::Error::msg("Unable to verify access token because the provided 'kid' claim in the access token does not match any known public keys"))
            }
        }?;

        let mut default_headers = reqwest::header::HeaderMap::new();
        let authorization_header_string: String = format!("Bearer {}", access_token.as_ref());
        let authorization_header_value: reqwest::header::HeaderValue =
            reqwest::header::HeaderValue::from_str(&authorization_header_string)?;
        default_headers.insert(reqwest::header::AUTHORIZATION, authorization_header_value);
        default_headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        #[allow(unused_mut)]
        let mut client_builder = reqwest::Client::builder().default_headers(default_headers);
        #[cfg(feature = "deflate")]
        {
            client_builder = client_builder.deflate(true);
        }
        #[cfg(feature = "gzip")]
        {
            client_builder = client_builder.gzip(true)
        }
        #[cfg(feature = "brotli")]
        {
            client_builder = client_builder.brotli(true)
        }
        let client = client_builder.build()?;
        Ok(Self {
            client,
            base_url,
            access_token: access_token.as_ref().to_string(),
        })
    }
}

// pub mod actions {
//     #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
//     #[serde(rename_all = "camelCase")]
//     pub struct GetActionsOptions {
//         pub trigger_id: Option<String>,
//         pub action_name: Option<String>,
//         pub deployed: Option<bool>,
//         pub per_page: Option<i32>,
//         pub installed: Option<bool>,
//     }
//     pub struct Actions(Vec<Action>);
//     pub struct Action;
//     impl super::Client {
//         pub async fn get_actions(
//             &self,
//             _options: GetActionsOptions,
//         ) -> Result<Actions, anyhow::Error> {
//             todo!()
//         }
//     }
//     impl std::default::Default for GetActionsOptions {
//         fn default() -> Self {
//             Self {
//                 action_name: None,
//                 trigger_id: None,
//                 deployed: None,
//                 per_page: None,
//                 installed: None,
//             }
//         }
//     }
// }
