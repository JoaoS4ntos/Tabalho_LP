use diesel::deserialize::Queryable;
use diesel::prelude::Insertable;
use diesel::query_builder::AsChangeset;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use diesel::QueryDsl;
use crate::schema::users;
use crate::schema::files;
use diesel::prelude::*;


#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
    pub phone: &'a str,
}

#[derive(Queryable, Debug, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub phone: String,
}

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile<'a> {
    pub filename: &'a str,
    pub file_data: &'a [u8],
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct File {
    pub id: i32,
    pub filename: String,
    pub file_data: Vec<u8>,
}

pub fn get_user_by_username(conn: &mut PgConnection, username: &str) -> Option<User> {
    use crate::schema::users::dsl as users_dsl; // Usando um alias único para evitar conflitos

    match users_dsl::users.filter(users_dsl::username.eq(username))
        .first(conn) {
            Ok(user) => Some(user),
            Err(_) => None,
        }
}