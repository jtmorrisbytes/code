use std::path::PathBuf;

use rocket::{
    fairing::{Fairing, Info, Kind},
    request::{self, FromRequest},
    tokio::io::AsyncReadExt,
    Build, Request, Rocket, State,
};
// use serde::{Deserialize, Serialize};
use crate::frontend::{FrontendApp, FrontendAppProperties};

#[derive(Clone)]
pub struct FrontendTemplatePath(pub PathBuf);

pub struct FrontendTemplatePathFairing;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::async_trait]
impl Fairing for super::FrontendTemplatePathFairing {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            kind: Kind::Ignite,
            name: "Login Page Template",
        }
    }
    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        let figment = rocket.figment();
        let template_path = match figment.extract_inner::<PathBuf>("template_dir") {
            Ok(template_path) => template_path,
            Err(e) => {
                tracing::error!("Failed to get a pathbuf for template_dir in Rocket Config! {e}");
                return Err(rocket);
            }
        }
        .join("frontend.html");
        rocket::fairing::Result::Ok(rocket.manage(FrontendTemplatePath(template_path)))
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::async_trait]
impl<'r> FromRequest<'r> for FrontendTemplatePath {
    type Error = String;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let template_path = request
            .guard::<&State<FrontendTemplatePath>>()
            .await
            .expect("TemplatePath");
        // let figment = request.rocket().figment();
        // let template_path = figment.extract_inner::<PathBuf>("template_dir").unwrap().join("frontend.html");
        request::Outcome::Success(template_path.inner().to_owned())
    }
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub async fn server_render(
    template_path: FrontendTemplatePath,
    props: FrontendAppProperties,
    initial_data: String,
) -> Result<String, anyhow::Error> {
    let title = props.title.clone();
    let server_renderer: yew::ServerRenderer<FrontendApp> =
        yew::ServerRenderer::with_props(|| props);
    let html = {
        let body = server_renderer.render().await;
        let mut html = String::new();
        let _ = rocket::tokio::fs::File::open(template_path.0)
            .await?
            .read_to_string(&mut html)
            .await?;
        html = html
            .replace("{{title}}", &title)
            .replace("{{root}}", &body)
            .replace("{{initial_data}}", &initial_data);
        let mut minifer = html_minifier::HTMLMinifier::new();
        minifer.set_minify_code(true);
        minifer.set_remove_comments(false);
        minifer.digest(html)?;
        let html = String::from_utf8_lossy(minifer.get_html()).to_string();
        html
    };
    Ok(html)
}
