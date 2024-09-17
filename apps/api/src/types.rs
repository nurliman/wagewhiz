use crate::{
    entities::{people::Model as People, user_accounts::Model as UserAccount},
    errors::AppError,
};
use async_graphql::{InputObject, OutputType, SimpleObject};
use serde::{Deserialize, Serialize};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Serialize, SimpleObject)]
#[graphql(concrete(name = "PeoplePaginationResponse", params(People)))]
pub struct PaginationResponse<T: OutputType> {
    pub items: Vec<T>,
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Serialize, SimpleObject)]
pub struct Credential {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, SimpleObject)]
pub struct AuthResponse {
    pub user: UserAccount,
    pub credential: Credential,
}

#[derive(Deserialize, InputObject)]
pub struct LoginInput {
    #[graphql(validator(min_length = 1))]
    pub username: String,
    #[graphql(validator(min_length = 1))]
    pub password: String,
}

#[derive(Deserialize, InputObject)]
pub struct RefreshTokenInput {
    pub refresh_token: Option<String>,
}
