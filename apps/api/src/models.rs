use crate::{
    schema,
    utils::serde::{uuid_string, uuid_string_option},
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use time::{serde::rfc3339, Date, OffsetDateTime};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = schema::people)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    #[serde(with = "uuid_string")]
    pub id: Uuid,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
    pub name: Option<String>,
    pub nip: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub zip_code: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birthday: Option<Date>,
    pub organization: Option<String>,
    pub role: Option<String>,
    pub department: Option<String>,
    pub joining_date: Option<Date>,
}

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = schema::user_accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserAccount {
    #[serde(with = "uuid_string")]
    pub id: Uuid,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: String,
    #[serde(with = "uuid_string_option")]
    pub person_id: Option<Uuid>,
}

#[derive(Deserialize, Validate)]
pub struct SignIn {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RefreshToken {
    #[validate(required, length(min = 1, message = "Refresh token cannot be empty"))]
    pub refresh_token: Option<String>,
}

#[derive(Serialize)]
pub struct Credential {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct UserWithCredential {
    #[serde(flatten)]
    pub user: UserAccount,
    pub credential: Credential,
}

#[derive(Serialize, Deserialize)]
pub struct JobTitle {
    pub id: usize,
    pub name: String,
}
