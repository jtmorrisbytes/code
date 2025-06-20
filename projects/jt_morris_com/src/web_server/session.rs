pub use super::db::session::SessionError;
use time;

use core::str;
use time::format_description::well_known::Iso8601;

// #[cfg(feature="serialize",derive)]
#[cfg_attr(any(feature = "debug", debug_assertions), derive(Debug))]
#[allow(unused)]
pub struct Session(super::db::session::Session);

// default session length is five minutes for security
pub const DEFAULT_SESSION_MAX_DURATION: time::Duration = time::Duration::minutes(5);
pub const SESSION_MAX_DURATION: &str = "session.max_duration_seconds";

pub const SESSION_COOKIE_NAME: &str = "session";

// stay logged in for up to five minutes. Every request keeps you logged in for up to five minutes

fn try_parse_cookie_value(s: &str) -> Result<(time::OffsetDateTime, i64), anyhow::Error> {
    let index = s
        .find(".")
        .ok_or_else(|| anyhow::Error::msg("Failed to find index of '.' in session cookie value"))?;
    let (left, right) = s.split_at(index);
    let date = time::OffsetDateTime::parse(left, &Iso8601::DEFAULT)?;
    let session_id = right.parse()?;
    Ok((date, session_id))
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for Session {
    type Error = SessionError;
    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        use time::format_description::well_known::Iso8601;
        // the longest a user can be logged into
        let session_duration: time::Duration = request
            .rocket()
            .figment()
            .extract_inner(SESSION_MAX_DURATION)
            .unwrap_or(DEFAULT_SESSION_MAX_DURATION);
        // get a copy of the database pool from rocket state
        let pool = request
            .guard::<&rocket::State<super::db::PrimaryDatabasePool>>()
            .await
            .unwrap();
        let mut connection = match pool.get().await {
            Ok(c) => c,
            Err(e) => {
                return rocket::request::Outcome::Error((
                    rocket::http::Status::InternalServerError,
                    SessionError::DatabaseError(e.to_string()),
                ))
            }
        };
        // get the session id from a cookie jar
        let cookie_jar = request.cookies();
        let cookie = cookie_jar.get_private(SESSION_COOKIE_NAME);

        if cookie.is_none() {
            return rocket::request::Outcome::Error((
                rocket::http::Status::InternalServerError,
                SessionError::CookieNotFound,
            ));
        }
        let mut session_cookie = cookie.unwrap();
        let (mut cookietime, session_id) = match try_parse_cookie_value(session_cookie.value()) {
            Ok(v) => v,
            Err(e) => {
                return rocket::request::Outcome::Error((
                    rocket::http::Status::InternalServerError,
                    SessionError::CookieParseError(e.to_string()),
                ))
            }
        };
        // ensure offset is in utc time
        cookietime = cookietime.to_offset(time::UtcOffset::UTC);
        // check the offset date tame
        let now = time::OffsetDateTime::now_utc();

        // if the current time is newer than the session expiry time. Ex. cookie created at 10:00. set to expire at 10:05. the current time is 10:06. now (10:06) > cookietime (10:05)
        if now > cookietime {
            // delete the session in another thread before returning
            let _ = super::db::session::Session::delete_by_id(session_id, &mut connection).await.map_err(|e| {
                tracing::error!("Failed to delete session from database after it expired. This may leave the database session dangling.'{e}'")
            }).ok();
            // cookie expired
            return rocket::request::Outcome::Error((
                rocket::http::Status::Unauthorized,
                SessionError::Expired,
            ));
        }
        // refresh the expiry time
        let expires = now + session_duration;
        session_cookie.set_value(format!(
            "{}.{}",
            expires
                .format(&Iso8601::DEFAULT)
                .expect("session format expires failed"),
            session_id
        ));
        session_cookie.set_expires(expires);

        let mut session =
            match super::db::session::Session::get_by_id(session_id, &mut connection).await {
                Ok(Some(session)) => session,
                Ok(None) => {
                    return rocket::request::Outcome::Error((
                        rocket::http::Status::InternalServerError,
                        SessionError::NotFound,
                    ))
                }
                Err(database_error) => {
                    return rocket::request::Outcome::Error((
                        rocket::http::Status::InternalServerError,
                        SessionError::DatabaseError(database_error.to_string()),
                    ))
                }
            };

        match session.set_expires(expires, &mut connection).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Failed to update the session expiry time. This may result in the session expiring sooner than inteded {e}");
            }
        }
        rocket::request::Outcome::Success(Self(session))
    }
}
