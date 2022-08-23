use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::bot_credentials;

type QueryError = diesel::result::Error;

#[derive(Queryable, Serialize, Debug)]
pub struct BotCredentials {
    pub id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
}

#[derive(Insertable)]
#[table_name = "bot_credentials"]
pub struct NewBotCredentials<'a> {
    pub id: &'a str,
    pub access_token: &'a str,
    pub refresh_token: &'a str,
    pub user_id: &'a str,
}

pub struct CreateBotCredentials {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
}

#[derive(AsChangeset)]
#[table_name = "bot_credentials"]
pub struct UpdateBotCredentials<'a> {
    pub access_token: &'a str,
    pub refresh_token: &'a str,
}

pub fn update_bot_credentials(
    db: &SqliteConnection,
    id: &str,
    update_data: UpdateBotCredentials,
) -> Result<usize, QueryError> {
    diesel::update(bot_credentials::table.find(id))
        .set(&update_data)
        .execute(db)
}

pub fn create_bot_credentials(
    db: &SqliteConnection,
    credentials: &CreateBotCredentials,
) -> Result<BotCredentials, QueryError> {
    let uuid = format!("{}", Uuid::new_v4());
    let new_credentials = NewBotCredentials {
        id: &uuid,
        access_token: &credentials.access_token,
        refresh_token: &credentials.refresh_token,
        user_id: &credentials.user_id,
    };

    diesel::insert_into(bot_credentials::table)
        .values(&new_credentials)
        .execute(db)
        .and_then(|_| {
            bot_credentials::table
                .find(&uuid)
                .get_result::<BotCredentials>(db)
        })
}

pub fn get_bot_credentials_by_user_id(
    db: &SqliteConnection,
    user_id: &str,
) -> Result<BotCredentials, diesel::result::Error> {
    bot_credentials::table
        .filter(bot_credentials::user_id.eq(user_id))
        .first::<BotCredentials>(db)
}

pub fn get_or_create_bot_credentials(
    db: &SqliteConnection,
    bot_credentials: CreateBotCredentials,
) -> Result<BotCredentials, diesel::result::Error> {
    get_bot_credentials_by_user_id(db, &bot_credentials.user_id)
        .or_else(|_| create_bot_credentials(db, &bot_credentials))
}
