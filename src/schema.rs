// @generated automatically by Diesel CLI.

diesel::table! {
    credential_types (id) {
        id -> Uuid,
        #[max_length = 16]
        code -> Varchar,
        #[max_length = 16]
        name -> Varchar,
    }
}

diesel::table! {
    credentials (id) {
        id -> Uuid,
        credential_type_id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        salt -> Varchar,
        #[max_length = 255]
        hashed -> Varchar,
        #[max_length = 255]
        token -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 128]
        email -> Varchar,
        #[max_length = 64]
        username -> Varchar,
    }
}

diesel::joinable!(credentials -> credential_types (credential_type_id));
diesel::joinable!(credentials -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    credential_types,
    credentials,
    users,
);
