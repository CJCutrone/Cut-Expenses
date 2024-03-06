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
    users (id) {
        id -> Uuid,
        #[max_length = 128]
        email -> Varchar,
        #[max_length = 64]
        username -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    credential_types,
    users,
);
