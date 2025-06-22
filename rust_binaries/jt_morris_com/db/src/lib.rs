#![deny(
    unused_import_braces,
    unused_allocation,
    unused_assignments,
    unused_variables
)]
#![deny(warnings)]

// We are using diesel-async as our backend with diesel as the query builder. tls is provided by rustls,
// currently the only options are sslmode=none and sslmode-verify-full

pub mod _url;
pub mod auth_state;
pub mod schema;
pub mod session;
pub mod transactions;
pub mod user;

pub use diesel;
#[cfg(feature = "not-wasm32-unknown-unknown")]
pub use diesel_async;

#[cfg(feature = "not-wasm32-unknown-unknown")]
pub type PgPool =
    diesel_async::pooled_connection::deadpool::Pool<diesel_async::pg::AsyncPgConnection>;
#[cfg(feature = "wasm32-unknown-unknown")]
pub type PgPool = ();

#[cfg(feature = "not-wasm32-unknown-unknown")]
pub type Connection = diesel_async::pg::AsyncPgConnection;
#[cfg(feature = "wasm32-unknown-unknown")]
pub type Connection = ();

#[cfg(feature = "not-wasm32-unknown-unknown")]
pub type PgPooledConnection =
    diesel_async::pooled_connection::deadpool::Object<diesel_async::pg::AsyncPgConnection>;
#[cfg(feature = "wasm32-unknown-unknown")]
pub type PgPooledConnection = ();

use diesel_migrations::embed_migrations;
pub use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// helper macros that determine which piece of code you need to run without writing all of it
// #[macro_export]
// macro_rules! q_execute {
//     ($query:expr,$connection:expr) => {{
//         #[cfg(feature = "db-sync-pooled")]
//         {
//             use diesel::RunQueryDsl;
//             $query.execute(&mut $connection)
//         }
//         #[cfg(feature = "db-async-pooled")]
//         {
//             use diesel_async::RunQueryDsl;
//             $query.execute(&mut $connection).await
//         }
//     }};
// }
// pub use q_execute;

// #[macro_export]
// /// calls query.get_result(&mut connection).await, but handles sync or async backend for us. blocks on the syncrounous backend
// macro_rules! get_result {
//     ($query:expr,$connection:expr) => {{
//         #[cfg(feature = "db-sync-pooled")]
//         {
//             use diesel::RunQueryDsl;
//             $query.get_result(&mut $connection)
//         }
//         #[cfg(feature = "db-async-pooled")]
//         {
//             use diesel_async::RunQueryDsl;
//             $query.get_result(&mut $connection).await
//         }
//     }};
// }
// pub use get_result;

// #[macro_export]
// /// calls query.get_result(&mut connection).await, but handles sync or async backend for us
// macro_rules! get_results {
//     ($query:expr,$connection:expr) => {{
//         #[cfg(feature = "db-sync-pooled")]
//         {
//             use diesel::RunQueryDsl;
//             $query.get_results(&mut $connection)
//         }
//         #[cfg("db-async-pooled")]
//         {
//             use diesel_async::RunQueryDsl;
//             $query.get_results(&mut $connection).await
//         }
//     }};
// }
// pub use get_results;

// #[macro_export]
// macro_rules! columns {
//     ($path:path)=>{
//         paste::paste!{crate::schema::$path}
//     };
//     ($($path:path),+) => {
//         paste::paste!{
//             ($(crate::schema::$path,)*)
//         }
//     };
// }
// // pub use columns;

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
        #[doc = "inserts and updates when id has a conflict"]
        macro_rules! upsert {
            ($values:expr) => {
                diesel::insert_into(paste::paste! {$_schema::table})
                    .values($values)
                    .on_conflict(paste::paste!($_schema::id))
                    .set($values)
            };
            ($values:expr,$returning:expr) => {
                diesel::insert_into(paste::paste! {$_schema::table})
                    .values($values)
                    .on_conflict(paste::paste!($_schema::id))
                    .set($values)
                    .returning($returning)
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

#[derive(Clone)]
pub struct PrimaryDatabasePool(PgPool);

#[cfg(feature = "not-wasm32-unknown-unknown")]
impl PrimaryDatabasePool {
    pub async fn get_connection(&self) -> Result<PrimaryDatabasePoolConnection, anyhow::Error> {
        Ok(PrimaryDatabasePoolConnection(self.0.get().await?))
    }
    /// creates a new instance of this type from a figment instance from the path 'databases.primary'.
    /// expects the path to conform to type crate::DatabaseConfig
    #[cfg(feature = "figment")]
    pub async fn try_new_from_figment(figment: &figment::Figment) -> Result<Self, anyhow::Error> {
        let url: url::Url = figment.extract_inner("databases.primary.url")?;
        let max_connections: usize = figment.extract_inner("databases.primary.max_connections")?;
        let pool = self::create_pool(url.as_str(), max_connections)
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
#[cfg(all(feature = "rocket", feature = "not-wasm32-unknown-unknown"))]
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
#[cfg(all(feature = "rocket", feature = "not-wasm32-unknown-unknown"))]
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

        match pool.get().await {
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

#[cfg(feature = "not-wasm32-unknown-unknown")]
#[allow(unused)]
pub async fn run_migrations(
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

// #[cfg(feature = "not-wasm32-unknown-unknown")]
// pub async fn insert_user_on_auth_callback(
//     pool: self::PgPool,
//     u: user::User,
// ) -> Result<user::User, anyhow::Error> {
//     if u.email.is_none() {
//         tracing::error!("Email is now a requied field. you need to promt the user for their email address");
//         return Err(anyhow::Error::msg("Email is a required field"));
//     }
//     let email = u.email.unwrap();
//     user::User::create_returning_self(pool, &u.auth0_user_id, &u.full_name, &email, u.picture.as_deref(), u.profile.as_deref(), u.username).await

// }
#[cfg(feature = "not-wasm32-unknown-unknown")]
pub async fn insert_auth_state_returning_id(
    connection: &mut PgPooledConnection,
    auth_state: auth_state::AuthState,
) -> Result<uuid::Uuid, anyhow::Error> {
    auth_state.insert_returning_id(connection).await
}
#[cfg(feature = "not-wasm32-unknown-unknown")]

pub async fn get_all_time_income_for_user_async() -> Result<(), anyhow::Error> {
    todo!()
}
#[cfg(feature = "not-wasm32-unknown-unknown")]

pub async fn get_all_time_income_for_user(
    pool: self::PgPool,
    user_id: &uuid::Uuid,
) -> Result<Option<bigdecimal::BigDecimal>, anyhow::Error> {
    use diesel::{ExpressionMethods, QueryDsl};
    use diesel_async::RunQueryDsl;

    let mut connection = pool.get().await?;
    schema::transactions::table
        .select(diesel::dsl::sum(schema::transactions::total_cost))
        .filter(
            schema::transactions::total_cost
                .gt(<bigdecimal::BigDecimal as bigdecimal::Zero>::zero()),
        )
        .filter(schema::transactions::user_id.eq(user_id))
        .get_result(&mut connection)
        .await
        .map_err(|e| anyhow::Error::new(e))
}
#[cfg(feature = "not-wasm32-unknown-unknown")]

pub async fn get_all_time_expenses_for_user(
    pool: self::PgPool,
    user_id: &uuid::Uuid,
) -> Result<Option<bigdecimal::BigDecimal>, anyhow::Error> {
    use diesel::{ExpressionMethods, QueryDsl};
    use diesel_async::RunQueryDsl;
    let mut connection = pool.get().await?;
    schema::transactions::table
        .select(diesel::dsl::sum(schema::transactions::total_cost))
        .filter(
            schema::transactions::total_cost
                .lt(<bigdecimal::BigDecimal as bigdecimal::Zero>::zero()),
        )
        .filter(schema::transactions::user_id.eq(user_id))
        .get_result(&mut connection)
        .await
        .map_err(|e| anyhow::Error::new(e))
}
// configures a deafult crypto provider. this function needs to be called once in main();
pub fn initialize_rustls() {
    let _ = rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();
}

#[cfg(feature = "not-wasm32-unknown-unknown")]
pub async fn create_pool(
    database_url: &str,
    max_connections: usize,
) -> Result<PgPool, anyhow::Error> {
    use diesel_async::{
        pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager, ManagerConfig},
        AsyncPgConnection,
    };
    tracing::info!("The async backend uses the equivalent of sslmode=verify-full. if you get connection errors, please double check your certificates");

    let mut manager_config: ManagerConfig<AsyncPgConnection> = ManagerConfig::default();
    manager_config.custom_setup = Box::new(establish_connection);
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new_with_config(
        database_url,
        manager_config,
    );

    let pool = Pool::builder(config).max_size(max_connections).build()?;

    Ok(pool)
}
#[cfg(feature = "not-wasm32-unknown-unknown")]
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
                let error = e.downcast::<String>().unwrap_or(Box::new(String::new()));
                tracing::error!("fn root_certs panicked! {error}");
                return diesel::ConnectionResult::Err(diesel::ConnectionError::BadConnection(
                    error.to_string(),
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
        // let rustls_config = rustls::ClientConfig::builder()
        //     .with_root_certificates(root_certs.to_owned())
        //     .with_no_client_auth();
        // let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        // let (client, conn) = tokio_postgres::connect(&database_url, tls)
        //     .await
        //     .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("Database connection: {e}");
            }
        });
        AsyncPgConnection::try_from(client).await
    };
    fut.boxed()
}

#[cfg(feature = "not-wasm32-unknown-unknown")]

pub async fn test_connection(pool: self::PgPool) -> Result<usize, anyhow::Error> {
    use diesel_async::RunQueryDsl;
    let mut connection = pool.get().await?;
    Ok(diesel::sql_query("SELECT TRUE")
        .execute(&mut connection)
        .await?)
}
