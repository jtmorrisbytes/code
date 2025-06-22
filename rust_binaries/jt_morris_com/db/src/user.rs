// use rocket::time as time;
// use diesel::prelude::*;
// use crate::schema;
use crate::{PgPool, PgPooledConnection};
use diesel_async::RunQueryDsl;
use garde::Validate;

#[derive(diesel::Queryable, diesel::Insertable, diesel::AsChangeset, PartialEq, Eq, Validate)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[diesel(table_name=super::schema::users)]
pub struct User {
    #[garde(skip)]
    pub id: uuid::Uuid,
    #[garde(skip)]
    pub auth0_user_id: String,
    #[garde(skip)]
    pub full_name: String,
    #[garde(email)]
    pub email: Option<String>,
    #[garde(skip)]
    pub picture: Option<String>,
    #[garde(url)]
    pub profile: Option<String>,
    #[garde(skip)]
    pub username: String,
    #[garde(phone_number)]
    pub phone_number: Option<String>,
}
impl User {
    // pub fn new(
    //     auth0_user_id: &str,
    //     full_name: &str,
    //     email: &str,
    //     phone_number: Option<&str>,
    //     picture: Option<&str>,
    //     profile: Option<&str>,
    //     username: String,
    // ) -> Self {
    //     Self {
    //         id: uuid::Uuid::new_v4(),
    //         auth0_user_id: auth0_user_id.to_string(),
    //         full_name: full_name.to_string(),
    //         email: Some(email.to_string()),
    //         phone_number: phone_number.map(|s| s.to_string()),
    //         picture: picture.map(|s| s.to_string()),
    //         profile: profile.map(|s| s.to_string()),
    //         username
    //     }
    // }
    // pub fn get_by_username(pool: PrimaryDatabasePool) {

    // }
}
/// a module containing frequently used queries. any query that can be seperated from the execution of the query should be placed here for
/// reusabillity
pub mod query_builder {
    use crate::schema::users::{auth0_user_id, id, table, username};
    #[diesel::dsl::auto_type]
    pub fn check_username_unique<'a>(username_: &'a str) -> _ {
        use diesel::{ExpressionMethods, QueryDsl};
        // if any username matches the string given above, return its id, otherwise return nothing
        table.select(id).filter(username.eq(username_))
    }
    #[diesel::dsl::auto_type]
    pub fn check_email_unique<'a>(email_: &'a str) -> _ {
        use diesel::{ExpressionMethods, QueryDsl};
        crate::schema::users::table
            .select(crate::schema::users::id)
            .filter(crate::schema::users::email.eq(email_))
    }
    #[diesel::dsl::auto_type]
    pub fn select_id() -> _ {
        use diesel::QueryDsl;
        table.select(id)
    }
    #[diesel::dsl::auto_type]
    pub fn get_user_id_from_auth0_user_id<'a>(id_: &'a str) -> _ {
        use diesel::{ExpressionMethods, QueryDsl};
        select_id().filter(auth0_user_id.eq(id_))
    }
    #[diesel::dsl::auto_type]
    pub fn get_user_from_auth0_user_id(id_: String) -> _ {
        use diesel::{ExpressionMethods, QueryDsl};
        table.filter(auth0_user_id.eq(id_))
    }
}
#[cfg(feature = "not-wasm32-unknown-unknown")]
impl User {
    // some validation methods
    /// performs database pre-validation. checks unique constraints
    async fn db_validate(pool: PgPool, email: &str, username: &str) -> Result<(), anyhow::Error> {
        let mut conn1 = pool.get().await?;
        let mut conn2 = pool.get().await?;

        // check constraints. dont insert if a username or email is taken. use upsert instead, but use caution.
        // you may want to display a message to the user that the email is taken or to choose another email.
        // we are doing this here in case we decide to use a database that doesnt support unique or check constraints.
        // its also helpful to have better error messages than "Input violates unique constraint ('constraint name')"

        // use tokio join here to hopefully run both queries at the same time
        let (email_result, username_result) = tokio::join! {
            Self::check_email_unique(email, &mut conn1),
            Self::check_username_unique(username, &mut conn2)
        };
        // we dont need these anymore
        drop(conn1);
        drop(conn2);

        // dont proceed if there is a database error
        let email_matches = email_result?;
        // check for matches and print and return a helpful error
        if let Some(id_list) = email_matches {
            // builds a list of matching uuids
            let matches = id_list
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join(",");
            let error_message = format!("One or more users with these ids: {matches} have matching usernames. Usernames are required to be unique values that map a user to their user id");
            tracing::error!(error_message);
            // dont proceed if the email already exists
            return Err(anyhow::Error::msg(error_message));
        }
        let username_matches = username_result?;
        // dont proceed if there is a database error
        if let Some(id_list) = username_matches {
            // builds a list of matching uuids
            let matches = id_list
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join(",");
            let error_message = format!("One or more users with these ids: {matches} have matching email addresses. Multiple users cannot have the same email address");
            tracing::error!(error_message);
            // dont proceed if the email already exists
            return Err(anyhow::Error::msg(error_message));
        }
        Ok(())
    }
    /// checks if there are any usernames in the database that match the given input and any matches will be returned as a list of user id's
    pub async fn check_username_unique(
        username: &str,
        connection: &mut PgPooledConnection,
    ) -> Result<Option<Vec<uuid::Uuid>>, diesel::result::Error> {
        use diesel::OptionalExtension;
        use diesel_async::RunQueryDsl;
        self::query_builder::check_username_unique(&username)
            .get_results(connection)
            .await
            .optional()
    }
    /// checks if there are any emails in the database that match the given input and any matches will be returned as a list of user id's
    pub async fn check_email_unique(
        email: &str,
        connection: &mut PgPooledConnection,
    ) -> Result<Option<Vec<uuid::Uuid>>, diesel::result::Error> {
        use diesel::OptionalExtension;
        use diesel_async::RunQueryDsl;
        self::query_builder::check_email_unique(&email)
            .get_results(connection)
            .await
            .optional()
    }
    /// performs an insert, returning the number of rows affected
    pub async fn create(
        pool: PgPool,
        auth0_user_id_: &str,
        full_name_: &str,
        email_: &str,
        phone_number_: Option<&str>,
        picture_: Option<&str>,
        profile_: Option<&str>,
        username_: String,
    ) -> Result<usize, anyhow::Error> {
        //run the db validation
        Self::db_validate(pool.clone(), email_, &username_).await?;
        // perform a struct level validaton before insert. the id here is just a dummy
        let mut _self = Self {
            id: uuid::Uuid::default(),
            auth0_user_id: auth0_user_id_.to_string(),
            full_name: full_name_.to_string(),
            email: Some(email_.to_string()),
            picture: picture_.map(|s| s.to_string()),
            profile: profile_.map(|s| s.to_string()),
            username: username_,
            phone_number: phone_number_.map(|s| s.to_string()),
        };
        _self.validate()?;

        // we have performed validation before insert. add additional validation above if required.
        use super::schema::users::{
            auth0_user_id, email, full_name, phone_number, picture, profile, table, username,
        };
        use diesel::ExpressionMethods;
        use diesel_async::RunQueryDsl;
        let mut connection = pool.get().await?;
        diesel::insert_into(table)
            .values((
                auth0_user_id.eq(auth0_user_id_),
                full_name.eq(full_name_),
                email.eq(email_),
                picture.eq(picture_),
                profile.eq(profile_),
                username.eq(_self.username),
                phone_number.eq(phone_number_),
            ))
            // .returning(all_columns)
            .execute(&mut connection)
            .await
            .map_err(|e| anyhow::Error::new(e))
    }
    /// performs an insert, returning the entire type
    pub async fn create_returning_self(
        pool: PgPool,
        auth0_user_id_: &str,
        full_name_: &str,
        email_: &str,
        phone_number_: Option<&str>,
        picture_: Option<&str>,
        profile_: Option<&str>,
        username_: String,
    ) -> Result<Self, anyhow::Error> {
        // run the db prevalidation
        Self::db_validate(pool.clone(), email_, &username_).await?;
        let mut conn1 = pool.get().await?;

        // perform a struct level validaton before insert. the id here is just a dummy
        let mut _self = Self {
            id: uuid::Uuid::default(),
            auth0_user_id: auth0_user_id_.to_string(),
            full_name: full_name_.to_string(),
            email: Some(email_.to_string()),
            picture: picture_.map(|s| s.to_string()),
            profile: profile_.map(|s| s.to_string()),
            username: username_,
            phone_number: phone_number_.map(|s| s.to_string()),
        };
        _self.validate()?;

        // we have performed validation before insert. add additional validation above if required.
        use super::schema::users::{
            all_columns, auth0_user_id, email, full_name, phone_number, picture, profile, table,
            username,
        };
        use diesel::ExpressionMethods;
        use diesel_async::RunQueryDsl;
        _self = diesel::insert_into(table)
            .values((
                auth0_user_id.eq(auth0_user_id_),
                full_name.eq(full_name_),
                email.eq(email_),
                picture.eq(picture_),
                profile.eq(profile_),
                username.eq(_self.username),
                phone_number.eq(phone_number_),
            ))
            .returning(all_columns)
            .get_result(&mut conn1)
            .await
            .map_err(|e| anyhow::Error::new(e))?;
        // perform a struct level validation again after insert (just to be cautious)
        _self.validate()?;
        Ok(_self)
    }
    // gets the user that matches this user id
    pub async fn get_id_from_auth0_user_id(
        connection: &mut PgPooledConnection,
        id: &str,
    ) -> Result<uuid::Uuid, anyhow::Error> {
        query_builder::get_user_id_from_auth0_user_id(id)
            .get_result(connection)
            .await
            .map_err(|e| anyhow::Error::new(e))
    }

    pub async fn get_from_auth0_user_id(
        connection: &mut PgPooledConnection,
        id: &str,
    ) -> Result<Self, anyhow::Error> {
        use super::schema::users::{auth0_user_id, table};
        use diesel::{ExpressionMethods, QueryDsl};
        use diesel_async::RunQueryDsl;
        let _self: Self = table
            .filter(auth0_user_id.eq(id))
            .get_result(connection)
            .await
            .map_err(|e| anyhow::Error::new(e))?;
        _self.validate()?;
        Ok(_self)
    }
    pub async fn upsert(self) -> Self {
        todo!()
    }
}
