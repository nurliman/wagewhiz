use super::users::get_user_by_username;
use crate::{
    env,
    errors::{self, AppError},
    models::{Credential, CredentialUser, User},
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rusty_paseto::prelude::*;
use time::format_description::well_known::Rfc3339;

pub async fn sign_in(username: &str, password: &str) -> Result<CredentialUser, AppError> {
    let user = get_user_by_username(username).await.map_err(|error| {
        if let AppError::UsernameNotFound(_) = error {
            return AppError::InvalidUsernameOrPassword;
        }
        error
    })?;

    if user.password.is_empty() {
        return Err(AppError::UserPasswordNotFound);
    }

    let is_password_valid = verify_password(password, &user.password).await?;

    if !is_password_valid {
        return Err(AppError::InvalidUsernameOrPassword);
    }

    let access_token = generate_access_token(&user).await?;
    let refresh_token = generate_refresh_token(&user).await?;
    let roles = get_roles_by_user(&user).await?;

    Ok(CredentialUser {
        id: user.id.clone().to_string(),
        username: username.to_owned(),
        roles,
        credential: Credential {
            access_token,
            refresh_token,
        },
    })
}

pub async fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|e| {
        tracing::error!("Error parsing password hash: {:?}", e);
        errors::AppError::InternalError
    })?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

async fn generate_token(
    user: &User,
    secret_key: &Key<64>,
    expiration: &str,
) -> anyhow::Result<String> {
    let key = PasetoAsymmetricPrivateKey::<V4, Public>::from(secret_key);

    let token = PasetoBuilder::<V4, Public>::default()
        .set_claim(ExpirationClaim::try_from(expiration)?)
        .set_claim(CustomClaim::try_from((
            "user_id",
            user.id.clone().to_string(),
        ))?)
        .build(&key)?;

    Ok(token)
}

async fn generate_access_token(user: &User) -> Result<String, AppError> {
    let env_var = env::get_env_var()?;
    let key = Key::<64>::from(env_var.access_token_secret_key.as_bytes());
    let in_15_minutes = (time::OffsetDateTime::now_utc() + time::Duration::minutes(15))
        .format(&Rfc3339)
        .map_err(|e| {
            tracing::error!("Error formatting date: {:?}", e);
            errors::AppError::InternalError
        })?;

    generate_token(user, &key, &in_15_minutes)
        .await
        .map_err(|e| {
            tracing::error!("Error generating access token: {:?}", e);
            errors::AppError::InternalError
        })
}

async fn generate_refresh_token(user: &User) -> Result<String, AppError> {
    let env_var = env::get_env_var()?;
    let key = Key::<64>::from(env_var.refresh_token_secret_key.as_bytes());
    let in_2_hours = (time::OffsetDateTime::now_utc() + time::Duration::hours(2))
        .format(&Rfc3339)
        .map_err(|e| {
            tracing::error!("Error formatting date: {:?}", e);
            errors::AppError::InternalError
        })?;

    generate_token(user, &key, &in_2_hours).await.map_err(|e| {
        tracing::error!("Error generating access token: {:?}", e);
        errors::AppError::InternalError
    })
}

async fn get_roles_by_user(_user: &User) -> Result<Vec<String>, AppError> {
    let mut roles = Vec::new();
    roles.push("admin".to_string());
    Ok(roles)
}
