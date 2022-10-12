use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::bot_credentials;

type QueryError = diesel::result::Error;

#[derive(Queryable, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BotCredentials {
    pub id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: Option<Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = bot_credentials)]
pub struct NewBotCredentials<'a> {
    pub access_token: &'a str,
    pub refresh_token: &'a str,
    pub user_id: &'a Uuid,
}

pub struct CreateBotCredentials {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: Uuid,
}

#[derive(AsChangeset)]
#[diesel(table_name = bot_credentials)]
pub struct UpdateBotCredentials<'a> {
    pub access_token: &'a str,
    pub refresh_token: &'a str,
}

pub fn update_bot_credentials(
    db: &mut PgConnection,
    bot_id: &Uuid,
    update_data: UpdateBotCredentials,
) -> Result<BotCredentials, QueryError> {
    diesel::update(bot_credentials::table.filter(bot_credentials::id.eq(bot_id)))
        .set(&update_data)
        .get_result::<BotCredentials>(db)
}

pub fn create_bot_credentials(
    db: &mut PgConnection,
    credentials: &CreateBotCredentials,
) -> Result<BotCredentials, QueryError> {
    let new_credentials = NewBotCredentials {
        access_token: &credentials.access_token,
        refresh_token: &credentials.refresh_token,
        user_id: &credentials.user_id,
    };

    diesel::insert_into(bot_credentials::table)
        .values(&new_credentials)
        .get_result::<BotCredentials>(db)
}

pub fn get_bot_credentials_by_user_id(
    db: &mut PgConnection,
    user_id: &Uuid,
) -> Result<BotCredentials, QueryError> {
    bot_credentials::table
        .filter(bot_credentials::user_id.eq(user_id))
        .first::<BotCredentials>(db)
}

pub fn get_or_create_bot_credentials(
    db: &mut PgConnection,
    bot_credentials: CreateBotCredentials,
) -> Result<BotCredentials, QueryError> {
    get_bot_credentials_by_user_id(db, &bot_credentials.user_id)
        .or_else(|_| create_bot_credentials(db, &bot_credentials))
}
