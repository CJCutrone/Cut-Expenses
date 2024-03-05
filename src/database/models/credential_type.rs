use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::credential_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CredentialType {
    pub id: Uuid,
    pub code: String,
    pub name: String
}