use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Serialize, Debug, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct BotCredentials {
    pub id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: Option<Uuid>,
}

pub struct CreateBotCredentials<'a> {
    pub access: &'a str,
    pub refresh: &'a str,
    pub user_id: &'a Uuid,
}

pub struct UpdateBotCredentials<'a> {
    pub id: &'a Uuid,
    pub access: &'a str,
    pub refresh: &'a str,
}

impl BotCredentials {
    pub async fn create(
        pool: &Pool<Postgres>,
        data: &CreateBotCredentials<'_>,
    ) -> sqlx::Result<BotCredentials> {
        sqlx::query_as!(
            BotCredentials,
            "
                INSERT INTO bot_credentials(access_token, refresh_token, user_id)
                VALUES ($1, $2, $3)
                RETURNING *;
            ",
            data.access,
            data.refresh,
            data.user_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &Pool<Postgres>,
        data: &UpdateBotCredentials<'_>,
    ) -> sqlx::Result<BotCredentials> {
        sqlx::query_as!(
            BotCredentials,
            "
                UPDATE bot_credentials
                SET (access_token, refresh_token) = ($1, $2)
                WHERE id = $3 RETURNING *;
            ",
            data.access,
            data.refresh,
            data.id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_user_id(
        pool: &Pool<Postgres>,
        user_id: &Uuid,
    ) -> sqlx::Result<Option<BotCredentials>> {
        sqlx::query_as!(
            BotCredentials,
            "
                SELECT * from bot_credentials
                    WHERE user_id = $1;
            ",
            user_id
        )
        .fetch_optional(pool)
        .await
    }
}
