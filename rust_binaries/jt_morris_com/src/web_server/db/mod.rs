#![deny(
    unused_import_braces,
    unused_allocation,
    unused_assignments,
    unused_variables
)]
#![deny(warnings)]

pub mod _url;
pub mod auth_state;
pub mod schema;
pub mod session;
pub mod transactions;
pub mod user;

use rocket::figment;

// We are handling both diesel and async_diesel backends for now, prefereing diesel async with tls
// prefer to build queries using query builder functions and use the macros to run the queries to help handle
// async <--> sync. you will still need to use async functions to call async, but hopefully you will not need to append '.await' to 200+ callsites

#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    not(feature = "db-sync-pooled"),
    not(feature = "db-async-pooled")
))]
compile_error!(
    "You must choose a database backend. Enable feature 'db-sync-pooled' or 'db-async-pooled'"
);

#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-sync-pooled",
    feature = "db-async-pooled"
))]
compile_error!("You cannot enable multiple backends at the same time. Enable feature 'db-sync-pooled' or 'db-async-pooled'");

// use std::fmt::Debug;
// use std::io::Write;
// use std::ops::Deref;
#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-sync-pooled"
))]
pub type Pool<Conn> = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<Conn>>;
#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-async-pooled"
))]

pub type Pool<Conn> = diesel_async::pooled_connection::deadpool::Pool<Conn>;
#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-sync-pooled"
))]
pub type PgPool = Pool<diesel::pg::PgConnection>;
#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-async-pooled"
))]
pub type PgPool = Pool<diesel_async::AsyncPgConnection>;

pub use bigdecimal::BigDecimal;
pub use diesel;

#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-sync-pooled"
))]
pub type Connection = diesel::pg::PgConnection;
#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-async-pooled"
))]
pub type Connection = diesel_async::pg::AsyncPgConnection;

#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-sync-pooled"
))]
pub type PgPooledConnection<Conn> =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::Pg>>;
#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-async-pooled"
))]
pub type PgPooledConnection =
    diesel_async::pooled_connection::deadpool::Object<diesel_async::AsyncPgConnection>;

use diesel_migrations::embed_migrations;
pub use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// helper macros that determine which piece of code you need to run without writing all of it
#[macro_export]
macro_rules! execute {
    ($query:expr,$connection:expr) => {{
        #[cfg(feature = "db-sync-pooled")]
        {
            use diesel::RunQueryDsl;
            $query.execute(&mut $connection)
        }
        #[cfg(feature = "db-async-pooled")]
        {
            use diesel_async::RunQueryDsl;
            $query.execute(&mut $connection).await
        }
    }};
}
pub use execute;

#[macro_export]
/// calls query.get_result(&mut connection), but handles sync or async backend for us. blocks on the syncrounous backend
macro_rules! get_result {
    ($query:expr,$connection:expr) => {{
        #[cfg(feature = "db-sync-pooled")]
        {
            use diesel::RunQueryDsl;
            $query.get_result(&mut $connection)
        }
        #[cfg(feature = "db-async-pooled")]
        {
            use diesel_async::RunQueryDsl;
            $query.get_result(&mut $connection).await
        }
    }};
}
pub use get_result;

#[macro_export]
/// calls query.get_result(&mut connection), but handles sync or async backend for us
macro_rules! get_results {
    ($query:expr,$connection:expr) => {{
        #[cfg(feature = "db-sync-pooled")]
        {
            use diesel::RunQueryDsl;
            $query.get_results(&mut $connection)
        }
        #[cfg("db-async-pooled")]
        {
            use diesel_async::RunQueryDsl;
            $query.get_results(&mut $connection).await
        }
    }};
}
pub use get_results;


#[macro_export]
macro_rules! columns {
    ($path:path)=>{
        paste::paste!{crate::web_server::db::schema::$path}
    };
    ($($path:path),+) => {
        paste::paste!{
            ($(crate::web_server::db::schema::$path,)*)
        }
    };
}
pub use columns;

#[macro_export]
///  A macro that implements queries for Create, Read,Update,and Delete.
///  it is almost impossible to use functions for some of this because the return type of
/// some queryies are literally paragraphs long! if you need a query that is not provided here,
/// simply import the schema and diesel traits as needed. note
///
/// $schema= the path to a diesel::table! macro implementing the table schema.
///
/// $type: a type that implements diesel::Queryable,Insertable,and AsChangeSet.
///
/// note that this macro just writes queries inline inside a scope block so you
/// can decide how you want the data afterward.
/// after you call this macro, its wise to use the other helper macros
/// in case you switch backends, or you will have to handle 'fn _' <--> 'async fn _' conversions
/// in more places!
///
///
macro_rules! implement_crud {
    ($_schema:path,$type:ty) => {
        #[macro_export]
        #[doc="inserts and updates when id has a conflict"]
        macro_rules! upsert {
            ($values:expr) => {
                diesel::insert_into(paste::paste!{$_schema::table}).values($values).on_conflict(paste::paste!($_schema::id)).set($values)
            };
            ($values:expr,$returning:expr) => {
                diesel::insert_into(paste::paste!{$_schema::table}).values($values).on_conflict(paste::paste!($_schema::id)).set($values).returning($returning)
            };
        }
        #[macro_export]
        #[doc = "Deletes using a unique identifier"]
        macro_rules! delete_by_id {
            ($id:expr,$returning:expr) => {{
                use diesel::ExpressionMethods;
                diesel::delete(paste::paste! {$_schema::table})
                    .filter(paste::paste! {$_schema::id.eq($id)})
                    .returning($returning)
            }};
            ($id:expr) => {{
                use diesel::ExpressionMethods;
                diesel::delete(paste::paste! {$_schema::table})
                    .filter(paste::paste! {$_schema::id.eq($id)})
            }};

        }
        pub use delete_by_id;
    };
}
// pub fn paste::paste!{upsert_$type:snake} -> (_value:$ty) {}
// pub fn paste::paste!{upsert_many_$type:snake}(_values:Vec<$ty>){}
pub use implement_crud;

#[derive(Clone)]
pub struct PrimaryDatabasePool(PgPool);

#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-async-pooled"
))]
impl PrimaryDatabasePool {
    pub async fn get_connection(&self) -> Result<PrimaryDatabasePoolConnection, anyhow::Error> {
        Ok(PrimaryDatabasePoolConnection(self.0.get().await?))
    }
    /// creates a new instance of this type from a figment instance from the path 'databases.primary'.
    /// expects the path to conform to type crate::DatabaseConfig
    pub async fn try_new_from_figment(figment: &figment::Figment) -> Result<Self, anyhow::Error> {
        let url: url::Url = figment.extract_inner("databases.primary.url")?;
        let max_connections: usize = figment.extract_inner("databases.primary.max_connections")?;
        let pool = self::create_pool::<diesel::pg::Pg>(url.as_str(), max_connections)
            .await
            .map_err(|e| {
                anyhow::Error::msg(e.to_string())
                    .context("While attempting to create the primary database pool")
            })?;
        Ok(Self(pool))
    }
}
impl PrimaryDatabasePool {
    pub fn inner(&self) -> self::PgPool {
        self.0.to_owned()
    }
    pub fn into_inner(self) -> self::PgPool {
        self.0.clone()
    }
}
impl std::ops::Deref for PrimaryDatabasePool {
    type Target = self::PgPool;
    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}
impl std::ops::DerefMut for PrimaryDatabasePool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut (self.0)
    }
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for PrimaryDatabasePool {
    type Error = anyhow::Error;
    async fn from_request(
        r: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let pool = match r.guard::<&rocket::State<Self>>().await {
            rocket::request::Outcome::Success(pools) => pools.inner(),
            rocket::request::Outcome::Error((status, _error)) => {
                return rocket::request::Outcome::Error((
                    status,
                    anyhow::Error::msg("Failed to extract primary database pool from state"),
                ));
            }
            rocket::request::Outcome::Forward(_) => unimplemented!(),
        };

        rocket::request::Outcome::Success(pool.to_owned())
    }
}

pub struct PrimaryDatabasePoolConnection(PgPooledConnection);
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for PrimaryDatabasePoolConnection {
    type Error = anyhow::Error;
    async fn from_request(
        r: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let pool = match r.guard::<&rocket::State<PrimaryDatabasePool>>().await {
            rocket::request::Outcome::Success(pool) => pool.inner(),
            rocket::request::Outcome::Error((status, _error)) => {
                return rocket::request::Outcome::Error((
                    status,
                    anyhow::Error::msg("Failed to extract primary database pool from state"),
                ));
            }
            rocket::request::Outcome::Forward(_) => unimplemented!(),
        };
        let result = {
            #[cfg(feature = "db-async-pooled")]
            {
                pool.get().await
            }
            #[cfg(feature = "db-sync-pooled")]
            {
                pool.get()
            }
        };
        match result {
            Ok(conn) => rocket::request::Outcome::Success(Self(conn)),
            Err(e) => rocket::request::Outcome::Error((
                rocket::http::Status::InternalServerError,
                anyhow::Error::new(e),
            )),
        }
    }
}
impl std::ops::Deref for PrimaryDatabasePoolConnection {
    type Target = PgPooledConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for PrimaryDatabasePoolConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[allow(unused)]
pub fn run_migrations(
    connection: &mut impl MigrationHarness<diesel::pg::Pg>,
) -> Result<(), anyhow::Error> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection
        .run_pending_migrations(MIGRATIONS)
        .map(|_| ())
        .map_err(|e| anyhow::Error::msg(e.to_string()))
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub fn insert_user_on_auth_callback(
    pool: self::PgPool,
    u: super::user::User,
) -> Result<(), anyhow::Error> {
    let mut connection = pool.get()?;
    diesel::insert_into(schema::users::table)
        .values(u)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .map(|_| ())
        .map_err(|e| anyhow::Error::new(e))
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub fn insert_auth_state_returning_id(
    pool: self::PgPool,
    a: super::db::auth_state::AuthState,
) -> Result<uuid::Uuid, anyhow::Error> {
    let mut connection = pool.get()?;
    diesel::insert_into(schema::auth_state::table)
        .values((
            schema::auth_state::id.eq(a.id),
            schema::auth_state::started.eq(a.started),
            schema::auth_state::return_url.eq(a.return_url),
            schema::auth_state::scope.eq(a.scope),
            schema::auth_state::redirect_url.eq(a.redirect_url.to_string()),
        ))
        .returning(schema::auth_state::id)
        .get_result(&mut connection)
        // .optional()
        .map_err(|e| anyhow::Error::new(e))
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]

pub async fn get_all_time_income_for_user_async() -> Result<(), anyhow::Error> {
    todo!()
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]

pub fn get_all_time_income_for_user(
    pool: self::PgPool,
    user_id: &uuid::Uuid,
) -> Result<Option<BigDecimal>, anyhow::Error> {
    let mut connection = pool.get()?;
    schema::transactions::table
        .select(diesel::dsl::sum(schema::transactions::total_cost))
        .filter(schema::transactions::total_cost.gt(<BigDecimal as bigdecimal::Zero>::zero()))
        .filter(schema::transactions::user_id.eq(user_id))
        .get_result(&mut connection)
        .map_err(|e| anyhow::Error::new(e))
}
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]

pub fn get_all_time_expenses_for_user(
    pool: self::PgPool,
    user_id: &uuid::Uuid,
) -> Result<Option<BigDecimal>, anyhow::Error> {
    let mut connection = pool.get()?;
    schema::transactions::table
        .select(diesel::dsl::sum(schema::transactions::total_cost))
        .filter(schema::transactions::total_cost.lt(<BigDecimal as bigdecimal::Zero>::zero()))
        .filter(schema::transactions::user_id.eq(user_id))
        .get_result(&mut connection)
        .map_err(|e| anyhow::Error::new(e))
}

// pub async fn create_pool(
//     database_url: &str,
//     max_connections: usize,
// ) -> Result<deadpool::Pool<Connection>, anyhow::Error> {
//     let mut config = ManagerConfig::default();
//     config.custom_setup = Box::new(establish_connection);
//     let manager = AsyncDieselConnectionManager::<Connection>::new_with_config(database_url, config);
//     let pool = diesel_async::pooled_connection::deadpool::Pool::builder(manager);
//     Ok(pool.max_size(max_connections).build()?)
// }

#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-sync-pooled"
))]
pub fn create_pool<Conn>(
    database_url: &str,
    max_connections: u32,
) -> Result<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<Conn>>, anyhow::Error>
where
    Conn: R2D2Connection + 'static,
{
    let manager = ConnectionManager::<Conn>::new(database_url);
    diesel::r2d2::Pool::builder()
        .test_on_check_out(true)
        .max_size(max_connections)
        .min_idle(Some(1))
        .idle_timeout(Some(std::time::Duration::from_secs(600)))
        .build(manager)
        .map_err(|e| anyhow::Error::new(e))
}
#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-async-pooled"
))]
pub async fn create_pool<Conn>(
    database_url: &str,
    max_connections: usize,
) -> Result<PgPool, anyhow::Error> {
    use diesel_async::{
        pooled_connection::{AsyncDieselConnectionManager, ManagerConfig},
        AsyncPgConnection,
    };
    tracing::info!("The async backend uses the equivalent of sslmode=verify-full. if you get connection errors, please double check your certificates");

    let mut manager_config: ManagerConfig<AsyncPgConnection> = ManagerConfig::default();
    manager_config.custom_setup = Box::new(establish_connection);
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new_with_config(
        database_url,
        manager_config,
    );

    let database_url: url::Url = url::Url::parse(database_url)?;
    let query_pairs: std::collections::HashMap<
        std::borrow::Cow<'_, str>,
        std::borrow::Cow<'_, str>,
    > = database_url
        .query_pairs()
        .fold(std::collections::HashMap::default(), |mut acc, (l, r)| {
            acc.insert(l, r);
            acc
        });
    let _sslmode = query_pairs
        .get("sslmode")
        .map(|s| s.to_owned())
        .unwrap_or_default();

    let pool = Pool::builder(config).max_size(max_connections).build()?;

    Ok(pool)
}
#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "unknown"),
    feature = "db-async-pooled"
))]
pub fn establish_connection(
    database_url: &str,
) -> futures_util::future::BoxFuture<diesel::ConnectionResult<diesel_async::pg::AsyncPgConnection>>
{
    use diesel::ConnectionError;
    use diesel_async::AsyncPgConnection;
    use futures_util::FutureExt;
    println!("config: {database_url}");

    /// loads the os certificate store. panics if it fails catching the error is not possible due to the api;
    fn root_certs() -> &'static rustls::RootCertStore {
        static CACHE: std::sync::OnceLock<rustls::RootCertStore> = std::sync::OnceLock::new();
        CACHE.get_or_init(|| {
            let mut roots = rustls::RootCertStore::empty();
            let certs = rustls_native_certs::load_native_certs()
                .expect("Failed to load native certificates!");
            let (added, ignored) = roots.add_parsable_certificates(certs);
            tracing::debug!("added {added} certs. ignored {ignored} certs");
            roots
        })
    }

    let fut = async move {
        // loading the root certs can fail. attempt to catch the panic and turn it into an error with a message
        let load_root_certs_result = std::panic::catch_unwind(|| root_certs());

        let root_certs = match load_root_certs_result {
            Ok(root_certs) => root_certs,
            Err(e) => {
                return diesel::ConnectionResult::Err(diesel::ConnectionError::BadConnection(
                    *e.downcast::<String>().unwrap_or_default(),
                ));
            }
        };
        let rustls_config = rustls::ClientConfig::builder()
            .with_root_certificates(root_certs.to_owned())
            .with_no_client_auth();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let (client, conn) = tokio_postgres::connect(&database_url, tls)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
        rocket::tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("Database connection: {e}");
            }
        });
        AsyncPgConnection::try_from(client).await
    };
    fut.boxed()
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]

pub async fn test_connection(pool: self::PgPool) -> Result<usize, anyhow::Error> {
    use diesel_async::RunQueryDsl;
    let mut connection = pool.get().await?;
    Ok(diesel::sql_query("SELECT TRUE").execute(&mut connection)?)
}
