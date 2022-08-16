use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub twitch_id: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub username: &'a str,
    pub twitch_id: &'a str,
}

pub struct CreateUser {
    pub username: String,
    pub twitch_id: String,
}

pub fn get_user_by_username(
    db: &SqliteConnection,
    username: &str,
) -> Result<User, diesel::result::Error> {
    users::table
        .filter(users::username.eq(username))
        .first::<User>(db)
}

pub fn get_user_by_id(
    db: &SqliteConnection,
    username: &str,
) -> Result<User, diesel::result::Error> {
    users::table.find(username).get_result::<User>(db)
}

pub fn create_user(
    db: &SqliteConnection,
    user: &CreateUser,
) -> Result<User, diesel::result::Error> {
    let uuid = format!("{}", Uuid::new_v4());
    let new_user = NewUser {
        id: &uuid,
        username: &user.username,
        twitch_id: &user.twitch_id,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(db)
        .expect("Cannot insert new user");

    users::table.find(&uuid).get_result::<User>(db)
}

pub fn get_or_create_user(db: &SqliteConnection, user: CreateUser) -> Result<User, diesel::result::Error> {
    let maybe_user = get_user_by_username(db, &user.username);

    match maybe_user {
        Ok(user) => Ok(user),
        Err(_) => create_user(db, &user),
    }
}
