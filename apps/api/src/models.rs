use crate::entities::user_accounts::Model as UserAccount;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Login {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RefreshToken {
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
