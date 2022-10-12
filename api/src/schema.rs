// @generated automatically by Diesel CLI.

diesel::table! {
    bot_credentials (id) {
        id -> Uuid,
        access_token -> Varchar,
        refresh_token -> Varchar,
        user_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    user_commands (id) {
        id -> Uuid,
        name -> Varchar,
        message -> Varchar,
        user_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        twitch_id -> Varchar,
    }
}

diesel::joinable!(bot_credentials -> users (user_id));
diesel::joinable!(user_commands -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    bot_credentials,
    user_commands,
    users,
);
