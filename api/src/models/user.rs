use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::users;

#[derive(Identifiable, Queryable, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub twitch_id: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub twitch_id: &'a str,
}

pub struct CreateUser {
    pub username: String,
    pub twitch_id: String,
}

pub fn get_user_by_username(
    db: &mut PgConnection,
    username: &str,
) -> Result<User, diesel::result::Error> {
    users::table
        .filter(users::username.eq(username))
        .first::<User>(db)
}

pub fn get_user_by_id(db: &mut PgConnection, id: &Uuid) -> Result<User, diesel::result::Error> {
    users::table.find(id).get_result::<User>(db)
}

pub fn create_user(
    db: &mut PgConnection,
    user: &CreateUser,
) -> Result<User, diesel::result::Error> {
    let new_user = NewUser {
        username: &user.username,
        twitch_id: &user.twitch_id,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(db)
}

pub fn get_or_create_user(
    db: &mut PgConnection,
    user: CreateUser,
) -> Result<User, diesel::result::Error> {
    let maybe_user = get_user_by_username(db, &user.username);

    match maybe_user {
        Ok(user) => Ok(user),
        Err(_) => create_user(db, &user),
    }
}
