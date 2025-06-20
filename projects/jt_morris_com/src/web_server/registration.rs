// use rocket::{
//     form::{Form, Strict},
//     State,
// };

// use rocket_dyn_templates::{context, Template};

// #[cfg(feature = "rocket")]
// use super::PublicBaseAbsoluteUri;

// sqlx::query!(
//     r#"INSERT INTO "users" (id,external_account_id,full_name,phone_number,email,profile,picture)
//         VALUES (gen_random_uuid(),$1,$2,$3,$4,$5,$6)
//         ON CONFLICT(external_account_id)
//         DO UPDATE SET
//         full_name=EXCLUDED.full_name,
//         phone_number=EXCLUDED.phone_number,
//         email=EXCLUDED.email,
//         profile=EXCLUDED.profile,
//         picture=EXCLUDED.picture"#,
//     access_token.claims().sub,
//     user_info.name,
//     user_info.phone_number,
//     user_info.email,
//     user_info.profile,
//     user_info.picture
// )
// .execute(&mut **connection)
// .await
// .map_err(|e| {
//     tracing::error!("Error while attempting perform upsert on a user column. '{e}'",);
//     rocket::response::Redirect::to(index_uri.clone())
// })?;

// let user_info: UserInfo = authentication_client
// .get_user_info(
//     &access_token,
//     authentication_client_backoff.0.lock().await.clone(),
// )
// .await
// .map_err(|e| {
//     tracing::error!("Failed to get UserInfo. {e}");
//     rocket::response::Redirect::to(index_uri.clone())
// // })?;
// #[rocket::get(
//     "/registration/start?<email>&<full_name>&<phone_number>&<language>",
//     format = "text/html"
// )]
// pub async fn start_registration(
//     pool: &State<sqlx::Pool<sqlx::Postgres>>,
//     // public_base_uri: &State<crate::PublicBaseAbsoluteUri>,
//     access_token: Result<crate::authentication::AccessToken, &str>,
//     authentication_client: &State<auth0::AuthenticationClient>,
//     authentication_client_backoff: &State<crate::authentication::AuthenticationClientBackoff>,
//     email: Option<&str>,
//     full_name: Option<&str>,
//     phone_number: Option<&str>,
//     language: Option<&str>,
//     server_config: &rocket::State<crate::ServerConfig>,
// ) -> Result<Template, rocket_dyn_templates::Template> {
//     // attempt to reauthorize if the access token did not work. this uses redirects and may cause a redirect loop if third party cookies stop working
//     let access_token = access_token.map_err(|e| {
//         crate::render_error_template(
//             &proc_macros::make_absolute_rocket_uri!(crate::render_index()),
//             // "Authorization Failure",
//             // "Authorization Failure: Login required",
//             Some("No access token was present in the request"),
//             e,
//         )
//     })?;
//     let user_info: Option<crate::authentication::UserInfo> = authentication_client
//         .get_user_info(
//             access_token.inner(),
//             authentication_client_backoff.0.lock().await.clone(),
//         )
//         .await
//         .map_err(|e| {
//             tracing::error!("{e}");
//             e
//         })
//         .ok();

//     // let u = ;
//     Ok(Template::render(
//         "start_registration",
//         context! {
//             support_email:server_config.support_email(),
//             full_name:full_name.or_else(|| {user_info.as_ref().map(|u| u.full_name())}),
//             phone_number: phone_number.or_else(|| user_info.as_ref().map(|u| {u.phone_number()}).flatten()),
//             email: email.or_else(|| user_info.as_ref().map(|u|u.email()).flatten()),
//             language: language.or_else(|| {user_info.as_ref().map(|u| u.locale()).flatten()}),
//             profile: user_info.as_ref().map(|u|u.profile()),
//             picture: user_info.as_ref().map(|u|u.picture())
//         },
//     ))
// }
// #[derive(rocket::FromForm)]
// #[warn(unused)]
// pub struct CreateOrUpdateUserForm {
//     full_name: String,
//     email: String,
//     phone_number: String,
//     language: String,
//     profile: Option<String>,
//     picture: Option<String>,
// }
// #[rocket::post(
//     "/registration/start",
//     format = "application/x-www-form-urlencoded",
//     data = "<body>"
// )]
// pub async fn create_or_update_user(
//     // public_base_uri: &State<crate::PublicBaseAbsoluteUri>,
//     access_token: Option<crate::authentication::AccessToken>,
//     body: Form<Strict<CreateOrUpdateUserForm>>,
//     pool: &State<sqlx::Pool<sqlx::Postgres>>,
//     server_config: &State<crate::ServerConfig>,
// ) -> rocket::response::Redirect {
//     let public_base_url: String = server_config.public_base_url().to_string();
//     if access_token.is_none() {
//         return rocket::response::Redirect::to(
//             public_base_url.clone() + &rocket::uri!(crate::render_index()).to_string(),
//         );
//     }
//     let access_token = access_token.unwrap();
//     // try an upsert, if it fails, just log the error
//     let _ = sqlx::query!(
//         r#"INSERT INTO "users" (id,auth0_user_id,full_name,phone_number,email,profile,picture)
//             VALUES (gen_random_uuid(),$1,$2,$3,$4,$5,$6)
//             ON CONFLICT(auth0_user_id)
//             DO UPDATE SET
//             full_name=EXCLUDED.full_name,
//             phone_number=EXCLUDED.phone_number,
//             email=EXCLUDED.email,
//             profile=EXCLUDED.profile,
//             picture=EXCLUDED.picture"#,
//         access_token.claims().sub,
//         body.full_name,
//         body.phone_number,
//         body.email,
//         body.profile,
//         body.picture
//     )
//     .execute(&mut **connection)
//     .await
//     .map_err(|e| {
//         tracing::error!("Error while attempting perform upsert on a user column. '{e}'",);
//         // rocket::response::Redirect::to(index_uri.clone())
//         e
//     })
//     .ok();
//     rocket::response::Redirect::to(
//         public_base_url + &rocket::uri!(create_or_confirm_subscription()).to_string(),
//     )
// }
// #[rocket::get("/registration/billing", format = "text/html")]
// pub fn create_or_confirm_subscription() -> Template {
//     Template::render("registration_billing", context! {})
// }
