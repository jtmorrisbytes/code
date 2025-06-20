use garde::Validate;
// use crate::server::db::PrimaryDatabasePool;
#[cfg(feature = "not-wasm32-unknown-unknown")]
use super::{schema, PgPooledConnection};
// #[cfg(feature="not-wasm32-unknown-unknown")]
// use diesel::query_builder::{BatchInsert, ValuesClause,InsertStatement};
use time;

#[derive(PartialEq, Eq, Clone, Debug, Validate)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(
    feature = "not-wasm32-unknown-unknown",
    derive(
        diesel::Queryable,
        diesel::Insertable,
        diesel::AsChangeset,
        diesel::AsExpression
    )
)]
#[cfg_attr(feature="not-wasm32-unknown-unknown",diesel(table_name=super::schema::auth_state))]
#[cfg_attr(feature="not-wasm32-unknown-unknown",diesel(sql_type=super::schema::auth_state::SqlType))]

pub struct AuthState {
    #[garde(skip)]
    pub id: uuid::Uuid,
    #[garde(skip)]
    pub started: time::OffsetDateTime,
    #[garde(skip)]
    pub scope: String,
    #[garde(url)]
    pub redirect_url: String,
    #[garde(url)]
    pub return_url: Option<String>,
}
impl AuthState {
    pub fn new(return_url: Option<&url::Url>, scope: &str, redirect_url: &url::Url) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            started: time::OffsetDateTime::now_utc(),
            return_url: return_url.map(|u| u.to_string()),
            scope: scope.to_string(),
            redirect_url: redirect_url.to_string(),
        }
    }
}

// contains commonly reused query_fragments
pub mod query_builder {
    //     use crate::schema::auth_state::{table,Id};
    //     #[diesel::dsl::auto_type]
    //     pub fn delete_by_id(id: uuid::Uuid) -> _ {
    //         use diesel::{QueryDsl,ExpressionMethods};
    //         use diesel::dsl::delete as Delete;
    //         table.delete()
    //     }
}
#[cfg(feature = "not-wasm32-unknown-unknown")]
impl AuthState {
    pub async fn insert(
        &self,
        connection: &mut PgPooledConnection,
    ) -> Result<usize, anyhow::Error> {
        use super::schema::auth_state;
        use diesel::ExpressionMethods;
        use diesel_async::RunQueryDsl;
        self.validate()?;

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
        let query_result = query.execute(connection).await;
        query_result.map_err(|e| anyhow::Error::new(e))
        // insert_auth_state_returning_id(connection, self.clone()).await
    }
    pub async fn delete_by_id_returning_self(
        connection: &mut PgPooledConnection,
        id: &uuid::Uuid,
    ) -> Result<Self, anyhow::Error> {
        use diesel::ExpressionMethods;
        use diesel_async::RunQueryDsl;

        let _self: Self = diesel::delete(schema::auth_state::table)
            .filter(schema::auth_state::id.eq(id))
            // .returning(schema::auth_state::all_columns)
            .get_result(connection)
            .await
            .map_err(|e| anyhow::Error::new(e).context("while deleting auth state by id"))?;
        _self.validate()?;
        Ok(_self)
    }
    pub async fn delete_by_id(
        connection: &mut PgPooledConnection,
        id: &uuid::Uuid,
    ) -> Result<usize, anyhow::Error> {
        use diesel::ExpressionMethods;
        use diesel_async::RunQueryDsl;

        diesel::delete(schema::auth_state::dsl::auth_state)
            .filter(schema::auth_state::id.eq(id))
            .execute(connection)
            .await
            .map_err(|e| anyhow::Error::new(e).context("while deleting auth state by id"))
    }
    pub async fn delete_older_than_duration(
        // pool: super::PgPool,
        connection: &mut super::Connection,
        duration: time::Duration,
    ) -> Result<usize, anyhow::Error> {
        use diesel::ExpressionMethods;
        use diesel_async::RunQueryDsl;

        let elapsed = time::OffsetDateTime::now_utc() - duration;
        // let mut connection = pool.get().await?;
        diesel::delete(schema::auth_state::dsl::auth_state)
            .filter(schema::auth_state::started.lt(elapsed))
            .execute(connection)
            .await
            .map_err(|e| {
                anyhow::Error::new(e).context("while deleting auth state older than 15 minutes")
            })
        // let duration = chrono::Duration::try_from(interval).map_err(|e| anyhow::Error::new(e).context("while attempting to convert an std time duration to a chrono duration object while attempting to delete an expired authentication state"));
    }
    pub async fn get_for_callback(
        connection: &mut super::PgPooledConnection,
        id: uuid::Uuid,
    ) -> Result<Option<GetForCallback>, anyhow::Error> {
        use diesel::{ExpressionMethods, OptionalExtension};
        use diesel_async::RunQueryDsl;

        let delete_result: Option<GetForCallback> =
            diesel::delete(schema::auth_state::dsl::auth_state)
                .filter(schema::auth_state::id.eq(id))
                .returning((
                    schema::auth_state::scope,
                    schema::auth_state::redirect_url,
                    schema::auth_state::return_url,
                ))
                .get_result(connection)
                .await
                .optional()
                .map_err(|e| anyhow::Error::new(e))?;
        match delete_result {
            Some(_self) => {
                _self.validate()?;
                Ok(Some(_self))
            }
            None => Ok(None),
        }
    }
    pub async fn insert_returning_id(
        self,
        connection: &mut super::PgPooledConnection,
    ) -> Result<uuid::Uuid, anyhow::Error> {
        use diesel::ExpressionMethods;
        use diesel_async::RunQueryDsl;
        self.validate()?;
        diesel::insert_into(schema::auth_state::table)
            .values((
                schema::auth_state::id.eq(self.id),
                schema::auth_state::started.eq(self.started),
                schema::auth_state::return_url.eq(self.return_url),
                schema::auth_state::scope.eq(self.scope),
                schema::auth_state::redirect_url.eq(self.redirect_url.to_string()),
            ))
            .returning(schema::auth_state::id)
            .get_result(connection)
            .await
            // .optional()
            .map_err(|e| anyhow::Error::new(e))
    }
}

#[derive(PartialEq, Eq, Clone, Validate)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "not-wasm32-unknown-unknown", derive(diesel::Queryable))]
#[cfg_attr(not(all(target_arch="wasm32",target_os="unknown")),diesel(table_name=schema::auth_state))]
pub struct GetForCallback {
    #[garde(skip)]
    pub scope: String,
    #[garde(url)]
    pub redirect_url: String,
    #[garde(url)]
    pub return_url: Option<String>,
}
