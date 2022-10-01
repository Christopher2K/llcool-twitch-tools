use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::user_commands;

use super::user::User;

type QueryError = diesel::result::Error;

#[derive(Identifiable, Queryable, Associations, Serialize, Debug, AsChangeset)]
#[belongs_to(User)]
#[diesel(table_name = user_commands)]
#[serde(rename_all = "camelCase")]
pub struct UserCommand {
    pub id: Uuid,
    pub name: String,
    pub message: String,
    pub user_id: Option<Uuid>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user_commands)]
pub struct NewUserCommand {
    pub name: String,
    pub message: String,
    pub user_id: Option<Uuid>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = user_commands)]
pub struct UpdateUserCommand {
    pub name: String,
    pub message: String,
}

pub fn get_all_users_commands(
    db: &mut PgConnection,
    query_user_id: &Uuid,
) -> Result<Vec<UserCommand>, QueryError> {
    use crate::schema::user_commands::dsl::*;

    user_commands
        .filter(user_id.eq(query_user_id))
        .load::<UserCommand>(db)
}

pub fn create_user_command(
    db: &mut PgConnection,
    new_command: &NewUserCommand,
) -> Result<UserCommand, QueryError> {
    diesel::insert_into(user_commands::table)
        .values(new_command)
        .get_result(db)
}

pub fn update_user_command(
    db: &mut PgConnection,
    command_id: &Uuid,
    owner_id: &Uuid,
    updated_command: &UpdateUserCommand,
) -> Result<UserCommand, QueryError> {
    use crate::schema::user_commands::dsl::*;

    diesel::update(user_commands.filter(id.eq(command_id).and(user_id.eq(owner_id))))
        .set(updated_command)
        .get_result(db)
}

pub fn delete_user_command(
    db: &mut PgConnection,
    owner_id: &Uuid,
    command_id: &Uuid,
) -> Result<usize, QueryError> {
    use crate::schema::user_commands::dsl::*;

    diesel::delete(user_commands.filter(id.eq(command_id).and(user_id.eq(owner_id)))).execute(db)
}
