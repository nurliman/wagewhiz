use crate::{
    entities::user_accounts::Model as UserAccount,
    errors::AppError,
    repositories::auth::AuthRepository,
    services::{token::TokenService, users::UsersService},
    types::{AppResult, AuthResponse, Credential},
};
use nject::injectable;

#[injectable]
pub struct AuthService {
    auth_repo: AuthRepository,
    user_service: UsersService,
    token_service: TokenService,
}

impl AuthService {
    pub async fn login(&self, username: &str, password: &str) -> AppResult<AuthResponse> {
        let user =
            self.user_service
                .find_by_username(username)
                .await
                .map_err(|error| match error {
                    AppError::ResourceNotFound(_) => {
                        AppError::ValidationError("Invalid username or password".to_string())
                    }
                    _ => error,
                })?;

        if user.password.is_empty() {
            tracing::error!("User with username {} has no password", username);
            return Err(AppError::ValidationError(
                "Invalid username or password".to_string(),
            ));
        }

        let is_password_valid = self
            .auth_repo
            .verify_password(password, &user.password)
            .await?;

        if !is_password_valid {
            return Err(AppError::ValidationError(
                "Invalid username or password".to_string(),
            ));
        }

        self.generate_auth_response(&user).await
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> AppResult<AuthResponse> {
        let user_id = self
            .token_service
            .verify_refresh_token(refresh_token)
            .await?;
        let user = self.user_service.find_by_id(user_id).await?;
        self.generate_auth_response(&user).await
    }

    async fn generate_auth_response(&self, user: &UserAccount) -> AppResult<AuthResponse> {
        let user = UserAccount {
            password: String::new(),
            ..user.clone()
        };
        let (access_token, refresh_token) = self.token_service.generate_tokens(&user).await?;
        Ok(AuthResponse {
            user: user.clone(),
            credential: Credential {
                access_token,
                refresh_token,
            },
        })
    }
}
