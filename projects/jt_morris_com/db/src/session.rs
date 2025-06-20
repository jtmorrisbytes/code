use time;
// use chrono::Utc;

#[derive(Clone)]
#[cfg_attr(any(feature = "debug", debug_assertions), derive(Debug))]
#[cfg_attr(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    derive(
        diesel::Queryable,
        diesel::Selectable,
        diesel::Insertable,
        diesel::AsChangeset
    )
)]
#[cfg_attr(not(all(target_arch="wasm32",target_os="unknown")),diesel(table_name=super::schema::sessions))]

pub struct Session {
    id: i64,
    expires: time::OffsetDateTime,
    data: serde_json::Value,
}
#[cfg(all(feature = "not-wasm32-unknown-unknown", feature = "rocket"))]
impl Session {
    pub async fn new<'r>(
        expires_in: &time::Duration,
        connection: &mut super::PgPooledConnection,
        cookie_jar: &rocket::http::CookieJar<'r>,
    ) -> Result<Self, anyhow::Error> {
        use super::schema::sessions;
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;
        use time::format_description::well_known::Iso8601;
        let expires: time::OffsetDateTime = time::OffsetDateTime::now_utc() + *expires_in;
        let _self: Session = diesel::insert_into(sessions::table)
            .values((
                sessions::expires.eq(expires),
                sessions::data.eq(serde_json::Value::Object(serde_json::Map::new())),
            ))
            .returning((sessions::id, sessions::expires, sessions::data))
            .get_result(connection)
            .await?;

        // create a session cookie and add it to the cookie jar
        let date_string = _self.expires.format(&Iso8601::DEFAULT)?;
        let cookie_value = format!("{date_string}.{}", _self.id);
        let mut cookie = rocket::http::Cookie::new("session", cookie_value);
        cookie.set_expires(_self.expires);
        cookie_jar.add_private(cookie);

        Ok(_self)
    }
}
#[cfg(feature = "not-wasm32-unknown-unknown")]
impl Session {
    /// Performs an update, then returns the connection object
    pub async fn update(
        &self,
        connection: &mut super::PgPooledConnection,
    ) -> Result<(), anyhow::Error> {
        use super::schema::sessions;
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;
        let _ = diesel::update(sessions::table)
            .set(self)
            .filter(sessions::id.eq(self.id))
            .execute(connection)
            .await?;
        Ok(())
    }
    pub async fn get_by_id(
        id: i64,
        connection: &mut super::PgPooledConnection,
    ) -> Result<Option<Self>, anyhow::Error> {
        use super::schema::sessions;
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        Ok(sessions::table
            .filter(sessions::id.eq(id))
            .get_result(connection)
            .await
            .optional()?)
        // compile_error!("TODO: finish get_by_id for web_server_lib::db::session::Session")
    }
    pub async fn is_expired(
        &self,
        connection: &mut super::PgPooledConnection,
    ) -> Result<bool, anyhow::Error> {
        use super::schema::sessions;
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;
        Ok(sessions::table
            .select(sessions::expires.gt(diesel::dsl::now))
            .filter(sessions::id.eq(self.id))
            .get_result(connection)
            .await?)
    }
    pub async fn delete_by_id(
        id: i64,
        connection: &mut super::PgPooledConnection,
    ) -> Result<(), anyhow::Error> {
        use super::schema::sessions;
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;
        let _ = diesel::delete(sessions::table)
            .filter(sessions::id.eq(id))
            .execute(connection)
            .await?;
        Ok(())
    }
    /// performs a delete using id
    pub async fn delete(
        &self,
        connection: &mut super::PgPooledConnection,
    ) -> Result<(), anyhow::Error> {
        let _ = Self::delete_by_id(self.id, connection).await?;
        Ok(())
    }
    async fn update_expiry_by_id(
        id: i64,
        expires: time::OffsetDateTime,
        connection: &mut super::PgPooledConnection,
    ) -> Result<usize, anyhow::Error> {
        use super::schema::sessions;
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;
        Ok(diesel::update(sessions::table)
            .set(sessions::expires.eq(expires))
            .filter(sessions::id.eq(id))
            .execute(connection)
            .await?)
    }
    pub async fn refresh(
        &mut self,
        duration: time::Duration,
        connection: &mut super::PgPooledConnection,
    ) -> Result<usize, anyhow::Error> {
        let expires = time::OffsetDateTime::now_utc() + duration;
        let n = Self::update_expiry_by_id(self.id, expires, connection).await?;
        self.expires = expires;
        Ok(n)
    }
    pub async fn set_expires(
        &mut self,
        expires: time::OffsetDateTime,
        connection: &mut super::PgPooledConnection,
    ) -> Result<usize, anyhow::Error> {
        let n = Self::update_expiry_by_id(self.id, expires, connection).await?;
        self.expires = expires;
        Ok(n)
    }
    pub fn get_value<T: for<'de> serde::Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Option<T>, anyhow::Error> {
        let object = match self.data.as_object() {
            Some(o) => o,
            None => return Ok(None),
        };
        let value = match object.get(key) {
            Some(v) => v,
            None => return Ok(None),
        };
        Ok(serde_json::from_value(value.to_owned()).map_err(|e| anyhow::Error::new(e))?)
    }
    pub fn set_value<T: serde::Serialize>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<(), anyhow::Error> {
        let default_object = &mut serde_json::Map::new();
        let object = self.data.as_object_mut().unwrap_or(default_object);
        let value = serde_json::to_value(value)?;
        object[key] = value;
        self.data = serde_json::Value::from(object.to_owned());
        Ok(())
        // let mut o = self.data.as_object().map(|o|o.to_owned()).unwrap_or(serde_json::Map::new());
    }
}
impl Session {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn expires(&self) -> time::OffsetDateTime {
        self.expires
    }
}
// SESSION
#[derive(thiserror::Error, Debug)]
#[error("Session Error: {0}")]
pub enum SessionError {
    #[error("Expired")]
    Expired,
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("Failed to parse the session cookie")]
    CookieParseError(String),
    #[error("ExpiryParseFailed")]
    ExpiryParseFailed,
    #[error("Session not found")]
    NotFound,
    #[error("Session cookie not found")]
    CookieNotFound,
}
