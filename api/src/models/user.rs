use serde::Serialize;
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Serialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub twitch_id: String,
}

pub struct CreateUser<'a> {
    pub username: &'a str,
    pub twitch_id: &'a str,
}

impl User {
    pub async fn create(pool: &Pool<Postgres>, data: &CreateUser<'_>) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            "
                INSERT INTO users(username, twitch_id)
                VALUES ($1, $2)
                RETURNING *;
            ",
            data.username,
            data.twitch_id,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &Pool<Postgres>, id: &Uuid) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(
            User,
            "
                SELECT * FROM users
                WHERE id = $1;
            ",
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_by_username(
        pool: &Pool<Postgres>,
        username: &str,
    ) -> sqlx::Result<Option<User>> {
        sqlx::query_as!(
            User,
            "
                SELECT * FROM users
                WHERE username = $1;
            ",
            username
        )
        .fetch_optional(pool)
        .await
    }
}
