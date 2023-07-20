use crate::{errors::AppError, models, services, validation::JsonOrForm};
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
