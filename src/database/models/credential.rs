use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::credentials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Credential {
    pub id: Uuid,
    pub credential_type_id: Uuid,
    pub user_id: Uuid,
    pub salt: String,
    pub hashed: String,
    pub token: String,
    pub created_at: NaiveDateTime,
}