// @generated automatically by Diesel CLI.

diesel::table! {
    api_keys (id) {
        id -> Uuid,
        workspace_id -> Uuid,
        name -> Varchar,
        hash -> Varchar,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    channels (id) {
        id -> Uuid,
        workspace_id -> Uuid,
        code -> Varchar,
        description -> Varchar,
        vars -> Jsonb,
        enabled -> Bool,
    }
}

diesel::table! {
    health (id) {
        id -> Int4,
    }
}

diesel::table! {
    message_type_versions (id) {
        id -> Uuid,
        number -> Int4,
        schema -> Jsonb,
        vars -> Jsonb,
        enabled -> Bool,
        message_type_id -> Uuid,
    }
}

diesel::table! {
    message_types (id) {
        id -> Uuid,
        code -> Varchar,
        description -> Varchar,
        vars -> Jsonb,
        enabled -> Bool,
        channel_id -> Uuid,
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
        owner -> Bool,
        password_id -> Uuid,
    }
}

diesel::table! {
    workspaces (id) {
        id -> Uuid,
        code -> Varchar,
    }
}

diesel::joinable!(api_keys -> workspaces (workspace_id));
diesel::joinable!(channels -> workspaces (workspace_id));
diesel::joinable!(message_type_versions -> message_types (message_type_id));
diesel::joinable!(message_types -> channels (channel_id));
diesel::joinable!(users -> passwords (password_id));
diesel::joinable!(users -> workspaces (workspace_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    channels,
    health,
    message_type_versions,
    message_types,
    passwords,
    users,
    workspaces,
);
