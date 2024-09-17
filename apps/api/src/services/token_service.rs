use crate::{
    entities::user_accounts::Model as UserAccount, errors::AppError,
    repositories::auth::AuthRepository, types::AppResult,
};
use nject::injectable;
use pasetors::token::TrustedToken;
use uuid::Uuid;

#[injectable]
pub struct TokenService {
    auth_repo: AuthRepository,
}

impl TokenService {
    pub async fn generate_tokens(&self, user: &UserAccount) -> AppResult<(String, String)> {
        let (access_token, refresh_token) = tokio::join!(
            self.auth_repo.generate_access_token(user),
            self.auth_repo.generate_refresh_token(user),
        );
        Ok((access_token?, refresh_token?))
    }

    pub async fn verify_refresh_token(&self, refresh_token: &str) -> AppResult<Uuid> {
        let verified_token = self.auth_repo.verify_refresh_token(refresh_token).await?;
        Self::extract_user_id_from_token(&verified_token)
    }

    pub async fn verify_access_token(&self, access_token: &str) -> AppResult<Uuid> {
        let verified_token = self.auth_repo.verify_access_token(access_token).await?;
        Self::extract_user_id_from_token(&verified_token)
    }

    fn extract_user_id_from_token(verified_token: &TrustedToken) -> AppResult<Uuid> {
        let token_claims = verified_token
            .payload_claims()
            .ok_or(AppError::Unauthenticated)?;
        let token_subject = token_claims
            .get_claim("sub")
            .and_then(|v| v.as_str())
            .ok_or(AppError::Unauthenticated)?;
        let user_id = Uuid::parse_str(token_subject).map_err(|_| AppError::Unauthenticated)?;
        Ok(user_id)
    }
}
