
// use rocket::time as time;
// use diesel::prelude::*;
// use crate::schema;

use crate::web_server::authorization::AccessTokenRequestError;

#[derive(diesel::Queryable, diesel::Insertable, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[diesel(table_name=super::schema::users)]
pub struct User {
    pub id: uuid::Uuid,
    pub auth0_user_id: String,
    pub full_name: String,
    pub email: Option<String>,
    // pub phone_number: Option<String>,
    pub picture: Option<String>,
    pub profile: Option<String>,
    pub username: String
}
impl User {
    pub fn new(
        auth0_user_id: &str,
        full_name: &str,
        email: &str,
        _phone_number: Option<&str>,
        picture: Option<&str>,
        profile: Option<&str>,
        username: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            auth0_user_id: auth0_user_id.to_string(),
            full_name: full_name.to_string(),
            email: Some(email.to_string()),
            // phone_number: phone_number.map(|s| s.to_string()),
            picture: picture.map(|s| s.to_string()),
            profile: profile.map(|s| s.to_string()),
            username
        }
    }
    // pub fn get_by_username(pool: PrimaryDatabasePool) {

    // }
}

pub fn boxable_query<'a>() -> super::schema::users::BoxedQuery<'a,diesel::pg::Pg> {
    use super::schema::users;
    use diesel::prelude::QueryDsl;
    users::table.into_boxed()

}
#[diesel::dsl::auto_type]
pub fn SelectId() -> _ {
    use diesel::QueryDsl;
    super::schema::users::table.select(super::schema::users::id)
}


#[diesel::dsl::auto_type]
pub fn GetUserIdFromAuth0UserId(id:String) -> _ {
    use diesel::{QueryDsl,ExpressionMethods};
    SelectId().filter(super::schema::users::auth0_user_id.eq(id))
}
#[diesel::dsl::auto_type]
pub fn GetUserFromAuth0Id(id:String) ->_ {
    use diesel::{QueryDsl,ExpressionMethods};
    super::schema::users::table.filter(super::schema::users::auth0_user_id.eq(id))
}



// #[cfg(not(all(target_arch="wasm32",target_os="unknown")))]
// impl User {
// pub fn get_id_from_auth0_user_id<'a>(
//     auth0_user_id: impl AsRef<str>,
// ) -> super::schema::users::BoxedQuery<'a,diesel::pg::Pg> {
//     use super::schema;
//     // use schema::users::dsl::*;
//     schema::users::dsl::users
//         .select(schema::users::id)
//         .filter(schema::users::auth0_user_id.eq(auth0_user_id.as_ref()))
// }
// pub fn get_by_auth0_user_id<'a>(
//     id: &str
// ) -> super::schema::users::BoxedQuery<'a,diesel::pg::Pg> {
//     use super::schema;
//     use diesel::QueryDsl;
//     schema::users::dsl::users
//         .filter(schema::users::auth0_user_id.eq(id))
//         .into_boxed()
// }
// }
// super::derive_run_dsl!(super::schema::users,User);

