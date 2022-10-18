use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Serialize, Clone, FromRow)]
pub struct UserCommand {
    pub id: Uuid,
    pub name: String,
    pub message: String,
    pub user_id: Uuid,
}

pub struct CreateUserCommand<'a> {
    pub name: &'a str,
    pub message: &'a str,
    pub user_id: &'a Uuid,
}

pub struct UpdateUserCommand<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
    pub message: &'a str,
    pub user_id: &'a Uuid,
}

#[derive(Deserialize)]
pub struct UserCommandPayload {
    pub name: String,
    pub message: String,
}

impl UserCommand {
    pub async fn create(
        pool: &Pool<Postgres>,
        data: &CreateUserCommand<'_>,
    ) -> sqlx::Result<UserCommand> {
        sqlx::query_as!(
            UserCommand,
            "
                INSERT INTO user_commands(name, message, user_id)   
                VALUES($1, $2, $3)
                RETURNING *;
            ",
            data.name,
            data.message,
            data.user_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_all_by_user_id(
        pool: &Pool<Postgres>,
        user_id: &Uuid,
    ) -> sqlx::Result<Vec<UserCommand>> {
        sqlx::query_as!(
            UserCommand,
            "
                SELECT c.*
                FROM user_commands c 
                JOIN users u ON u.id = c.user_id 
                WHERE u.id = $1;
            ",
            user_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update(
        pool: &Pool<Postgres>,
        data: &UpdateUserCommand<'_>,
    ) -> sqlx::Result<UserCommand> {
        sqlx::query_as!(
            UserCommand,
            "
                UPDATE user_commands
                SET (name, message) = ($1, $2)
                WHERE id = $3
                    AND user_id = $4
                RETURNING *;
            ",
            data.name,
            data.message,
            data.id,
            data.user_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(
        pool: &Pool<Postgres>,
        user_id: &Uuid,
        id: &Uuid,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "
                DELETE FROM user_commands
                WHERE id = $1
                    AND user_id = $2;
            ",
            id,
            user_id,
        )
        .execute(pool)
        .await
    }
}
