use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Queryable, Serialize, Deserialize)]
#[diesel(table_name = global_commands)]
pub struct GlobalCommand {
    pub id: Uuid,
    pub command_definition: CommandDefinition,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum CommandDefinition {
    Pattern { pattern: String, message: String },
    Plain { name: String, message: String },
}

pub fn get_global_commands(db: &mut PgConnection) {
    use crate::schema::bot_credentials;


}
