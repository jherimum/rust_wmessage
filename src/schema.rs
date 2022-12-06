// @generated automatically by Diesel CLI.

diesel::table! {
    api_keys (id) {
        id -> Uuid,
        workspace_id -> Uuid,
        name -> Varchar,
        prefix -> Varchar,
        hash -> Varchar,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    channels (id) {
        id -> Uuid,
        workspace_id -> Uuid,
        code -> Varchar,
        name -> Varchar,
        properties -> Jsonb,
    }
}

diesel::table! {
    message_type_versions (id) {
        id -> Uuid,
        message_type_id -> Uuid,
        number -> Int4,
    }
}

diesel::table! {
    message_types (id) {
        id -> Uuid,
        channel_id -> Uuid,
        code -> Varchar,
        name -> Varchar,
    }
}

diesel::table! {
    passwords (id) {
        id -> Uuid,
        hash -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        workspace_id -> Uuid,
        password_id -> Uuid,
        owner -> Bool,
    }
}

diesel::table! {
    workspaces (id) {
        id -> Uuid,
        code -> Varchar,
    }
}

diesel::joinable!(channels -> workspaces (workspace_id));
diesel::joinable!(message_type_versions -> message_types (message_type_id));
diesel::joinable!(message_types -> channels (channel_id));
diesel::joinable!(users -> passwords (password_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    channels,
    message_type_versions,
    message_types,
    passwords,
    users,
    workspaces,
);
