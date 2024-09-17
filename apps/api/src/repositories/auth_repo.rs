use crate::{
    config::ConfigProvider,
    constants::{ACCESS_TOKEN_MAX_AGE, REFRESH_TOKEN_MAX_AGE},
    entities::user_accounts::Model as UserAccount,
    errors::AppError,
    types::AppResult,
    utils::paseto::{parse_key_data, KeyPurpose},
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use nject::injectable;
use pasetors::{
    claims::{Claims, ClaimsValidationRules},
    keys::{AsymmetricPublicKey, AsymmetricSecretKey},
    public,
    token::{TrustedToken, UntrustedToken},
};
use time::format_description::well_known::Rfc3339;

#[injectable]
pub struct AuthRepository {
    config_provider: ConfigProvider,
}

impl AuthRepository {
    pub async fn verify_password(&self, password: &str, password_hash: &str) -> AppResult<bool> {
        let parsed_hash = PasswordHash::new(password_hash).map_err(|e| {
            tracing::error!("Error parsing password hash: {:?}", e);
            AppError::InternalError
        })?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    async fn verify_token(&self, token: &str, public_key: &[u8]) -> AppResult<TrustedToken> {
        let key = AsymmetricPublicKey::from(public_key).map_err(|e| {
            tracing::error!("Error parsing public key: {:?}", e);
            AppError::InternalError
        })?;

        let validation_rules = ClaimsValidationRules::new();
        let untrusted_token = UntrustedToken::<pasetors::Public, pasetors::version4::V4>::try_from(
            token,
        )
        .map_err(|e| {
            tracing::error!("Error parsing token: {:?}", e);
            AppError::Unauthenticated
        })?;

        public::verify(&key, &untrusted_token, &validation_rules, None, None).map_err(|e| {
            tracing::error!("Error verifying token: {:?}", e);
            AppError::Unauthenticated
        })
    }

    pub async fn verify_access_token(&self, access_token: &str) -> AppResult<TrustedToken> {
        let config = self.config_provider.get_config();
        let public_key = parse_key_data(KeyPurpose::Public, &config.access_token_public_key, None)
            .map_err(|_| {
                tracing::error!("Error parsing access token public key");
                AppError::InternalError
            })?;

        self.verify_token(access_token, &public_key).await
    }

    pub async fn verify_refresh_token(&self, refresh_token: &str) -> AppResult<TrustedToken> {
        let config = self.config_provider.get_config();
        let public_key = parse_key_data(KeyPurpose::Public, &config.refresh_token_public_key, None)
            .map_err(|_| {
                tracing::error!("Error parsing refresh token public key");
                AppError::InternalError
            })?;

        self.verify_token(refresh_token, &public_key).await
    }

    async fn generate_token(
        &self,
        user: &UserAccount,
        secret_key: &[u8],
        expiration: &str,
    ) -> AppResult<String> {
        let key = AsymmetricSecretKey::from(secret_key).map_err(|e| {
            tracing::error!("Error parsing secret key: {:?}", e);
            AppError::InternalError
        })?;
        let mut claims = Claims::new().map_err(|e| {
            tracing::error!("Error creating claims: {:?}", e);
            AppError::InternalError
        })?;
        claims.expiration(expiration).map_err(|e| {
            tracing::error!("Error setting expiration: {:?}", e);
            AppError::InternalError
        })?;
        claims.subject(&user.id.to_string()).map_err(|e| {
            tracing::error!("Error setting subject: {:?}", e);
            AppError::InternalError
        })?;
        public::sign(&key, &claims, None, None).map_err(|e| {
            tracing::error!("Error signing token: {:?}", e);
            AppError::InternalError
        })
    }

    pub async fn generate_access_token(&self, user: &UserAccount) -> AppResult<String> {
        let config = self.config_provider.get_config();
        let key = parse_key_data(KeyPurpose::Secret, &config.access_token_secret_key, None)
            .map_err(|_| {
                tracing::error!("Error parsing access token secret key");
                AppError::InternalError
            })?;

        let expiration = (time::OffsetDateTime::now_utc() + ACCESS_TOKEN_MAX_AGE)
            .format(&Rfc3339)
            .map_err(|e| {
                tracing::error!("Error formatting date: {:?}", e);
                AppError::InternalError
            })?;

        self.generate_token(user, &key, &expiration).await
    }

    pub async fn generate_refresh_token(&self, user: &UserAccount) -> AppResult<String> {
        let config = self.config_provider.get_config();
        let key = parse_key_data(KeyPurpose::Secret, &config.refresh_token_secret_key, None)
            .map_err(|_| {
                tracing::error!("Error parsing refresh token secret key");
                AppError::InternalError
            })?;

        let expiration = (time::OffsetDateTime::now_utc() + REFRESH_TOKEN_MAX_AGE)
            .format(&Rfc3339)
            .map_err(|e| {
                tracing::error!("Error formatting date: {:?}", e);
                AppError::InternalError
            })?;

        self.generate_token(user, &key, &expiration).await
    }
}
