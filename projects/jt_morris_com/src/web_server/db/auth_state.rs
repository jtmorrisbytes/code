use super::_url::Url;
// use crate::server::db::PrimaryDatabasePool;
pub use crate::db::auth_state::AuthState;
#[cfg(all(not(target_arch="wasm32"),not(target_os="unknown")))]
use super::{schema, PgPooledConnection};
#[cfg(all(not(target_arch="wasm32"),not(target_os="unknown")))]
use diesel::query_builder::{BatchInsert, ValuesClause,InsertStatement};
use time;

#[derive(PartialEq, Clone, Eq, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
// #[cfg_attr(feature="debug",derive(Debug))]
#[cfg_attr(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    derive(
        diesel::prelude::QueryableByName,
        diesel::Insertable,
        diesel::AsExpression
    )
)]
#[cfg_attr(not(all(target_arch="wasm32",target_os="unknown")),diesel(table_name= super::schema::auth_state))]
#[cfg_attr(not(all(target_arch="wasm32",target_os="unknown")),diesel(sql_type=diesel::sql_types::Text))]

pub struct ReturnUrl(
    #[cfg_attr(
        not(all(target_arch = "wasm32", target_os = "unknown")),
        diesel(column_name = "redirect_url")
    )]
    pub(crate) Url,
);
impl std::ops::Deref for ReturnUrl {
    type Target = url::Url;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
impl std::convert::AsRef<url::Url> for ReturnUrl {
    fn as_ref(&self) -> &url::Url {
        self.0.as_ref()
    }
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::pg::Pg> for ReturnUrl {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        // self.0.to_sql(&mut out.reborrow())
        <super::_url::Url as diesel::serialize::ToSql<diesel::sql_types::Text,diesel::pg::Pg>>::to_sql(&self.0, &mut out.reborrow())
    }
}

#[derive(PartialEq, Clone, Eq, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
// #[cfg_attr(feature="debug",derive(Debug))]
#[cfg_attr(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    derive(diesel::prelude::QueryableByName, diesel::Insertable, diesel::AsExpression)
)]
#[cfg_attr(not(all(target_arch="wasm32",target_os="unknown")),diesel(table_name= super::schema::auth_state))]
#[cfg_attr(not(all(target_arch="wasm32",target_os="unknown")),diesel(sql_type=diesel::sql_types::Text))]

pub struct RedirectUrl(
    #[cfg_attr(
        not(all(target_arch = "wasm32", target_os = "unknown")),
        diesel(column_name = "return_url")
    )]
    pub(crate) Url,
);
impl std::ops::Deref for RedirectUrl {
    type Target = url::Url;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
impl std::convert::AsRef<url::Url> for RedirectUrl {
    fn as_ref(&self) -> &url::Url {
        self.0.as_ref()
    }
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::pg::Pg> for RedirectUrl {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        // self.0.to_sql(&mut out.reborrow())
        <super::_url::Url as diesel::serialize::ToSql<diesel::sql_types::Text,diesel::pg::Pg>>::to_sql(&self.0, &mut out.reborrow())
    }
}

#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    derive(diesel::Queryable, diesel::Insertable, diesel::AsChangeset)
)]
#[cfg_attr(not(all(target_arch="wasm32",target_os="unknown")),diesel(table_name=super::schema::auth_state))]
pub struct AuthState {
    pub id: uuid::Uuid,
    pub started: time::OffsetDateTime,
    pub return_url: Option<ReturnUrl>,
    pub scope: String,
    pub redirect_url: RedirectUrl,
}
impl AuthState {
    pub fn new(return_url: Option<&url::Url>, scope: &str, redirect_url: &url::Url) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            started: time::OffsetDateTime::now_utc(),
            return_url: return_url.map(|u| ReturnUrl(Url(u.to_owned()))),
            scope: scope.to_string(),
            redirect_url: RedirectUrl(Url(redirect_url.to_owned())),
        }
    }
}
#[macro_export]
macro_rules! _columns {
    ($path:path)=>{
        paste::paste!{crate::web_server::db::schema::auth_state::$path}
    };
    ($($path:path),+) => {
        paste::paste!{
            ($(crate::web_server::db::schema::auth_state::$path,)*)
        }
    };
}


// this only works if you use the fully qualified path?
super::implement_crud!(crate::web_server::db::schema::auth_state,AuthState);


#[derive(PartialEq, Clone, Eq, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
// #[cfg_attr(feature="debug",derive(Debug))]
#[cfg_attr(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    derive(diesel::prelude::QueryableByName)
)]
#[cfg_attr(not(all(target_arch="wasm32",target_os="unknown")),diesel(table_name=super::schema::auth_state))]
pub struct GetForCallback{
    scope:String,redirect_url:RedirectUrl,return_url:ReturnUrl
}


pub async fn get_for_callback(state:String, connection: &mut PgPooledConnection) -> Result<GetForCallback,anyhow::Error> {
        let returning = _columns!(scope,redirect_url,return_url);
        let query = self::delete_by_id!(state,returning);
        super::get_result!(query,connection)
}




// #[allow(non_snake_case)]
// it is impossable to name the type for this

#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-async-pooled"
))]
impl AuthState {
    pub async fn insert(
        &self,
        connection: &mut PgPooledConnection,
    ) -> Result<usize, anyhow::Error> {
        use super::schema::auth_state;
        use diesel::ExpressionMethods;
        let query = diesel::insert_into(auth_state::table)
            .values(self)
            .on_conflict(auth_state::id)
            .do_update()
            .set((
                schema::auth_state::started.eq(self.started.clone()),
                schema::auth_state::return_url.eq(self.return_url.clone()),
                schema::auth_state::scope.eq(&self.scope),
                schema::auth_state::redirect_url.eq(self.redirect_url.as_str()),
            ));
        let query_result = super::execute!(query, connection);
        query_result.map_err(|e| anyhow::Error::new(e))
        // insert_auth_state_returning_id(connection, self.clone()).await
    }
    pub fn delete_by_id(
        connection: &mut PgPooledConnection,
        id: &uuid::Uuid,
    ) -> Result<usize, anyhow::Error> {
        use super::schema::auth_state;
        use diesel::ExpressionMethods;
        diesel::delete(schema::auth_state::dsl::auth_state)
            .filter(schema::auth_state::id.eq(id))
            .execute(connection)
            .map_err(|e| anyhow::Error::new(e).context("while deleting auth state by id"))
    }
    pub fn delete_older_than_duration(
        pool: super::PgPool,
        duration: time::Duration,
    ) -> Result<usize, anyhow::Error> {
        let elapsed = time::OffsetDateTime::now_utc() - duration;
        let mut connection = pool.get()?;
        diesel::delete(schema::auth_state::dsl::auth_state)
            .filter(schema::auth_state::started.lt(elapsed))
            .execute(&mut connection)
            .map_err(|e| {
                anyhow::Error::new(e).context("while deleting auth state older than 15 minutes")
            })

        // let duration = chrono::Duration::try_from(interval).map_err(|e| anyhow::Error::new(e).context("while attempting to convert an std time duration to a chrono duration object while attempting to delete an expired authentication state"));
    }
    pub fn get_for_callback(
        connection: &mut super::PgPooledConnection,
        id: uuid::Uuid,
    ) -> Result<Option<AuthStateOnlyScopeRedirectUrlAndReturnUrl>, anyhow::Error> {
        diesel::delete(schema::auth_state::dsl::auth_state)
            .filter(schema::auth_state::id.eq(id))
            .returning((
                schema::auth_state::scope,
                schema::auth_state::redirect_url,
                schema::auth_state::return_url,
            ))
            .get_result(connection)
            .optional()
            .map_err(|e| anyhow::Error::new(e))
    }
}

#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    derive(Queryable)
)]
#[cfg_attr(not(all(target_arch="wasm32",target_os="unknown")),diesel(table_name=schema::auth_state))]
pub struct AuthStateOnlyScopeRedirectUrlAndReturnUrl {
    pub scope: String,
    pub redirect_url: String,
    pub return_url: Option<String>,
}
