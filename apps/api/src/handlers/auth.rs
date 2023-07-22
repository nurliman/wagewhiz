use crate::{
    errors::AppError,
    models::{CredentialUser, RefreshToken, SignIn},
    services,
    validation::JsonOrForm,
};
use axum::Json;

pub async fn sign_in(
    JsonOrForm(body): JsonOrForm<SignIn>,
) -> Result<Json<CredentialUser>, AppError> {
    let user = services::auth::sign_in(&body.username, &body.password).await?;
    Ok(Json(user))
}

pub async fn refresh_token(
    JsonOrForm(body): JsonOrForm<RefreshToken>,
) -> Result<Json<CredentialUser>, AppError> {
    let refresh_token = body.refresh_token.ok_or(AppError::InternalError)?;
    let user = services::auth::refresh_token(&refresh_token).await?;
    Ok(Json(user))
}
