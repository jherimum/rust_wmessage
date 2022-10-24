// @generated automatically by Diesel CLI.

diesel::table! {
    passwords (user_id) {
        user_id -> Uuid,
        hash -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        workspace_id -> Uuid,
        owner -> Bool,
    }
}

diesel::table! {
    workspaces (id) {
        id -> Uuid,
        code -> Varchar,
    }
}

diesel::joinable!(passwords -> users (user_id));
diesel::joinable!(users -> workspaces (workspace_id));

diesel::allow_tables_to_appear_in_same_query!(
    passwords,
    users,
    workspaces,
);
