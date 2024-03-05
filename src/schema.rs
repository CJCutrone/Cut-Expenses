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
