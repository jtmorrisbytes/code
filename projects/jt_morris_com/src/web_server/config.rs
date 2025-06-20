// use rocket::{form::ValueField, State};
// use std::{collections::HashMap, fmt::Write};
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use rocket::figment::{
    providers::{Env, Format, Toml},
    Figment, Profile,
};
pub const DEFAULT_DATABASE_MAX_CONNECTIONS: u32 = 20;
pub const PRIMARY_DATABASE_KEY: &str = "primary";
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ServerConfig {
    public_base_url: url::Url,
    auth0_domain: String,
    auth0_client_id: String,
    auth0_client_secret: String,
    auth0_audience: String,
    // database_url: url::Url,
    // database_max_connections: u32,

    // // #[cfg(feature)]
    // aws_s3_endpoint: url::Url,
    // aws_s3_access_key: String,
    // aws_s3_secret_key: String,
    public_files_dir: String,
    support_email: String,
    // template_files_root:String
}
// some of this code was yanked directly from rocket 0.5.1
/// Use this function to mimic configuration loading like the rocket does, except it only reads from the default profile.
/// this is needed to avoid direct dependencies on rocket itself in contexts like proc macros, small libraries, etc where
/// creating an entire server is not needed. otherwise use Rocket::figment().
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub fn rocket_figment() -> Figment {
    // use figment::providers::Format;
    Figment::new()
        .merge(Toml::file(Env::var_or("ROCKET_CONFIG", "Rocket.toml")).nested())
        .merge(Env::prefixed("ROCKET_").ignore(&["PROFILE"]).global())
        .select(Profile::from_env_or("ROCKET_PROFILE", Profile::Default))
}

impl ServerConfig {
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    pub fn try_new_from_figment(figment: &Figment) -> Result<Self, anyhow::Error> {
        let public_base_url: url::Url = figment.extract_inner("public_base_url")?;
        let auth0_domain: String = figment.extract_inner("auth0_domain")?;
        let auth0_client_id: String = figment.extract_inner("auth0_client_id")?;
        let auth0_client_secret: String = figment.extract_inner("auth0_client_secret")?;
        // let database_url: url::Url = figment.extract_inner("database_url")?;
        // let database_max_connections: u32 = figment
        //     .extract_inner::<u32>("database_max_connections")
        //     .ok()
        //     .unwrap_or(DEFAULT_DATABASE_MAX_CONNECTIONS);
        let auth0_audience: String = figment.extract_inner("auth0_audience")?;
        // let aws_s3_secret_key: String = figment.extract_inner("aws_s3_secret_key")?;
        // let aws_s3_access_key: String = figment.extract_inner("aws_s3_access_key")?;
        // let aws_s3_endpoint: url::Url = figment.extract_inner("aws_s3_endpoint")?;
        let support_email: String = figment.extract_inner("support_email")?;
        let public_files_dir: String = figment
            .extract_inner::<std::path::PathBuf>("public_files_dir")
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_else(|_| "./public".to_string());
        // let template_files_root: String = figment.extract_inner::<std::path::PathBuf>("template_files_root").map(|path| path.to_string_lossy().to_string()).unwrap_or_else(|_| "./templates".to_string());
        Ok(Self {
            public_base_url,
            auth0_client_id,
            auth0_domain,
            auth0_client_secret,
            auth0_audience,
            // aws_s3_access_key,
            // database_url,
            // database_max_connections,
            // aws_s3_endpoint,
            // aws_s3_secret_key,
            public_files_dir,
            // template_files_root,
            support_email,
        })
    }
    pub fn auth0_audience(&self) -> &str {
        &self.auth0_audience
    }
    pub fn support_email(&self) -> &str {
        &self.support_email
    }
    pub fn auth0_client_secret(&self) -> &str {
        &self.auth0_client_secret
    }
    pub fn public_base_url(&self) -> &url::Url {
        &self.public_base_url
    }
    pub fn auth0_domain(&self) -> &str {
        &self.auth0_domain
    }
    pub fn auth0_client_id(&self) -> &str {
        &self.auth0_client_id
    }
    // pub fn database_url(&self) -> &str {
    //     &self.database_url.as_str()
    // }
    // pub fn database_max_connections(&self) -> u32 {
    //     self.database_max_connections
    // }
    pub fn public_files_root(&self) -> &str {
        &self.public_files_dir
    }
    // pub fn template_files_root(&self) -> &str {
    //     &self.template_files_root
    // }
    // pub fn aws_s3_endpoint(&self) -> &str {
    //     &self.aws_s3_endpoint.as_str()
    // }
    // pub fn aws_s3_access_key(&self) -> &str {
    //     &self.aws_s3_access_key.as_str()
    // }
    // pub fn aws_s3_secret_key(&self) -> &str {
    //     &self.aws_s3_secret_key.as_str()
    // }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: url::Url,
    pub max_connections: usize,
}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DatabaseConfigs(std::collections::HashMap<String, DatabaseConfig>);
impl DatabaseConfigs {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn iter(&self) -> impl std::iter::Iterator<Item = (&String, &DatabaseConfig)> {
        self.0.iter()
    }
    pub fn into_iter(self) -> impl std::iter::Iterator<Item = (String, DatabaseConfig)> {
        self.0.into_iter()
    }
}
