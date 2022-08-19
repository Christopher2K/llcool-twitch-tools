table! {
    bot_credentials (id) {
        id -> Text,
        access_token -> Text,
        refresh_token -> Text,
        user_id -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        username -> Text,
        twitch_id -> Text,
    }
}

joinable!(bot_credentials -> users (user_id));

allow_tables_to_appear_in_same_query!(
    bot_credentials,
    users,
);
