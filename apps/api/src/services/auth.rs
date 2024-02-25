use super::users::{get_user_by_id, get_user_by_username};
use crate::{
    constants::{ACCESS_TOKEN_MAX_AGE, REFRESH_TOKEN_MAX_AGE},
    env,
    errors::AppError,
    models::{Credential, UserAccount, UserWithCredential},
    utils::paseto::{parse_key_data, KeyPurpose},
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use pasetors::{
    claims::{Claims, ClaimsValidationRules},
    keys::{AsymmetricPublicKey, AsymmetricSecretKey},
    public,
    token::{TrustedToken, UntrustedToken},
};
use time::format_description::well_known::Rfc3339;

pub async fn login(username: &str, password: &str) -> Result<UserWithCredential, AppError> {
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

    let (access_token, refresh_token) =
        tokio::join!(generate_access_token(&user), generate_refresh_token(&user),);

    Ok(UserWithCredential {
        user: UserAccount {
            id: user.id.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
            username: user.username.clone(),
            password: "".to_string(),
            role: user.role.clone(),
            person_id: user.person_id,
        },
        credential: Credential {
            access_token: access_token?,
            refresh_token: refresh_token?,
        },
    })
}

pub async fn refresh_token(refresh_token: &str) -> Result<UserWithCredential, AppError> {
    let verified_token = verify_refresh_token(refresh_token).await?;
    let token_claims = verified_token
        .payload_claims()
        .ok_or(AppError::TokenPayloadError)?;
    let token_subject = token_claims
        .get_claim("sub")
        .ok_or(AppError::TokenPayloadError)?;
    let token_subject_str = match token_subject {
        serde_json::Value::String(s) => s,
        _ => return Err(AppError::TokenPayloadError),
    };

    let user = get_user_by_id(token_subject_str).await?;

    let (access_token, refresh_token) =
        tokio::join!(generate_access_token(&user), generate_refresh_token(&user),);

    Ok(UserWithCredential {
        user: UserAccount {
            id: user.id.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
            username: user.username.clone(),
            password: "".to_string(),
            role: user.role.clone(),
            person_id: user.person_id,
        },
        credential: Credential {
            access_token: access_token?,
            refresh_token: refresh_token?,
        },
    })
}

pub async fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|e| {
        tracing::error!("Error parsing password hash: {:?}", e);
        AppError::InternalError
    })?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

// TODO: verify id user with token subject
pub async fn verify_token(public_token: &str, public_key: &[u8]) -> Result<TrustedToken, AppError> {
    let key = AsymmetricPublicKey::from(&public_key).map_err(|e| {
        tracing::error!("Error parsing access token public key: {:?}", e);
        AppError::InternalError
    })?;

    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token =
        UntrustedToken::<pasetors::Public, pasetors::version4::V4>::try_from(public_token)
            .map_err(|_| AppError::TokenParseError)?;

    let trusted_token = public::verify(&key, &untrusted_token, &validation_rules, None, None)
        .map_err(|_| AppError::InvalidToken)?;

    Ok(trusted_token)
}

pub async fn verify_access_token(access_token: &str) -> Result<TrustedToken, AppError> {
    let env_var = env::get_env_var()?;

    let public_key = parse_key_data(KeyPurpose::Public, &env_var.access_token_public_key, None)
        .map_err(|_| {
            tracing::error!("Error parsing access token public key");
            AppError::InternalError
        })?;

    verify_token(access_token, &public_key)
        .await
        .map_err(|error| {
            if let AppError::InvalidToken = error {
                return AppError::InvalidAccessToken;
            }
            error
        })
}

pub async fn verify_refresh_token(refresh_token: &str) -> Result<TrustedToken, AppError> {
    let env_var = env::get_env_var()?;

    let public_key = parse_key_data(KeyPurpose::Public, &env_var.refresh_token_public_key, None)
        .map_err(|_| {
            tracing::error!("Error parsing access token public key");
            AppError::InternalError
        })?;

    verify_token(refresh_token, &public_key)
        .await
        .map_err(|error| {
            if let AppError::InvalidToken = error {
                return AppError::InvalidRefreshToken;
            }
            error
        })
}

async fn generate_token(
    user: &UserAccount,
    secret_key: &[u8],
    expiration: &str,
) -> anyhow::Result<String> {
    let key = AsymmetricSecretKey::from(secret_key)?;

    let mut claims = Claims::new()?;
    claims.expiration(expiration)?;
    claims.subject(&user.id.clone().to_string())?;

    let token = public::sign(&key, &claims, None, None)?;

    Ok(token)
}

async fn generate_access_token(user: &UserAccount) -> Result<String, AppError> {
    let env_var = env::get_env_var()?;

    let key = parse_key_data(KeyPurpose::Secret, &env_var.access_token_secret_key, None).map_err(
        |_| {
            tracing::error!("Error parsing access token secret key");
            AppError::InternalError
        },
    )?;

    let expiration = (time::OffsetDateTime::now_utc() + ACCESS_TOKEN_MAX_AGE)
        .format(&Rfc3339)
        .map_err(|e| {
            tracing::error!("Error formatting date: {:?}", e);
            AppError::InternalError
        })?;

    generate_token(user, &key, &expiration).await.map_err(|e| {
        tracing::error!("Error generating access token: {:?}", e);
        AppError::InternalError
    })
}

async fn generate_refresh_token(user: &UserAccount) -> Result<String, AppError> {
    let env_var = env::get_env_var()?;

    let key = parse_key_data(KeyPurpose::Secret, &env_var.refresh_token_secret_key, None).map_err(
        |_| {
            tracing::error!("Error parsing refresh token secret key");
            AppError::InternalError
        },
    )?;

    let expiration = (time::OffsetDateTime::now_utc() + REFRESH_TOKEN_MAX_AGE)
        .format(&Rfc3339)
        .map_err(|e| {
            tracing::error!("Error formatting date: {:?}", e);
            AppError::InternalError
        })?;

    generate_token(user, &key, &expiration).await.map_err(|e| {
        tracing::error!("Error generating access token: {:?}", e);
        AppError::InternalError
    })
}
