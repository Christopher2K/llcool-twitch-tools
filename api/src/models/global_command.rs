use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct GlobalCommand {
    pub id: Uuid,
    pub command_definition: serde_json::Value,
}

#[derive(Clone, Debug, Serialize)]
pub struct UserGlobalCommand {
    pub id: Uuid,
    pub command_definition: serde_json::Value,
    pub is_activated: Option<bool>, // Will always be Option::Some
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum CommandDefinition {
    Pattern { pattern: String, message: String },
    Plain { name: String, message: String },
}

impl GlobalCommand {
    pub fn get_typed_definition(&self) -> Result<CommandDefinition, serde_json::Error> {
        serde_json::from_value(self.command_definition.clone())
    }

    pub async fn create(
        pool: &Pool<Postgres>,
        data: CommandDefinition,
    ) -> sqlx::Result<GlobalCommand> {
        sqlx::query_as!(
            GlobalCommand,
            "
                INSERT INTO global_commands(command_definition)
                VALUES($1)
                RETURNING *;
            ",
            serde_json::to_value(data).unwrap()
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_all(pool: &Pool<Postgres>) -> sqlx::Result<Vec<GlobalCommand>> {
        sqlx::query_as!(
            GlobalCommand,
            "
                SELECT *
                FROM global_commands;
            "
        )
        .fetch_all(pool)
        .await
    }

    pub async fn add_to_user(
        pool: &Pool<Postgres>,
        user_id: &Uuid,
        global_command_id: &Uuid,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "
                INSERT INTO users__global_commands(user_id, global_command_id)
                VALUES($1, $2);
            ",
            user_id,
            global_command_id,
        )
        .execute(pool)
        .await
    }

    pub async fn remove_from_user(
        pool: &Pool<Postgres>,
        user_id: &Uuid,
        global_command_id: &Uuid,
    ) -> sqlx::Result<PgQueryResult> {
        sqlx::query!(
            "
                DELETE FROM users__global_commands
                WHERE user_id = $1 AND global_command_id = $2;
            ",
            user_id,
            global_command_id,
        )
        .execute(pool)
        .await
    }

    pub async fn get_all_for_user(
        pool: &Pool<Postgres>,
        user_id: &Uuid,
    ) -> sqlx::Result<Vec<UserGlobalCommand>> {
        sqlx::query_as!(
            UserGlobalCommand,
            "
                SELECT
                    gc.id,
                    gc.command_definition,
                    ugc.user_id IS NOT NULL as is_activated
                FROM global_commands gc
                LEFT JOIN users__global_commands ugc ON gc.id = ugc.global_command_id
                    AND ugc.user_id = $1;
            ",
            user_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update(
        pool: &Pool<Postgres>,
        command_definition: CommandDefinition,
        global_command_id: &Uuid,
    ) -> sqlx::Result<GlobalCommand> {
        sqlx::query_as!(
            GlobalCommand,
            "
                UPDATE global_commands
                SET command_definition = $1
                WHERE id = $2
                RETURNING *;
            ",
            serde_json::to_value(command_definition).unwrap(),
            global_command_id
        )
        .fetch_one(pool)
        .await
    }
}
