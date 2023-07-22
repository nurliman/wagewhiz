use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, Date, OffsetDateTime};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[serde(with = "uuid::serde::compact")]
    pub id: Uuid,
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "iso8601")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "iso8601::option")]
    pub deleted_at: Option<OffsetDateTime>,
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = schema::people)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    #[serde(with = "uuid::serde::compact")]
    pub id: Uuid,
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "iso8601")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "iso8601::option")]
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
pub struct CredentialUser {
    pub id: String,
    pub username: String,
    pub roles: Vec<String>,
    pub credential: Credential,
}
