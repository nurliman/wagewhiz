use crate::{
    errors::{self, AppError},
    models, services,
    validation::JsonOrForm,
};
use axum::Json;

pub async fn list_users() -> Result<Json<models::User>, AppError> {
    let users = services::users::get_user_by_id("asd").await?;
    Ok(Json(users))
}

pub async fn sign_in(
    JsonOrForm(body): JsonOrForm<models::SignIn>,
) -> Result<Json<models::CredentialUser>, AppError> {
    let user = services::auth::sign_in(&body.username, &body.password).await?;
    Ok(Json(user))
}

pub async fn refresh_token(
    JsonOrForm(body): JsonOrForm<models::RefreshToken>,
) -> Result<Json<models::CredentialUser>, AppError> {
    let refresh_token = body.refresh_token.ok_or(errors::AppError::InternalError)?;
    let user = services::auth::refresh_token(&refresh_token).await?;
    Ok(Json(user))
}
