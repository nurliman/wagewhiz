use crate::{db, entities::user_accounts, errors::AppError};
use sea_orm::*;
use uuid::Uuid;

pub async fn get_user_by_username(username: &str) -> Result<user_accounts::Model, AppError> {
    let conn = db::get_connection().await?;

    let user = user_accounts::Entity::find()
        .filter(user_accounts::Column::Username.eq(username))
        .one(conn)
        .await
        .map_err(|e| {
            tracing::error!("Error getting user: {:?}", e);
            AppError::InternalError
        })?;

    if user.is_none() {
        return Err(AppError::UsernameNotFound(username.to_string()));
    }

    return Ok(user.unwrap());
}

pub async fn get_user_by_id(user_id: &str) -> Result<user_accounts::Model, AppError> {
    let conn = db::get_connection().await?;

    let user_uuid =
        Uuid::parse_str(user_id).map_err(|_| AppError::InvalidUuid(user_id.to_string()))?;

    let user = user_accounts::Entity::find_by_id(user_uuid)
        .one(conn)
        .await
        .map_err(|e| {
            tracing::error!("Error getting user: {:?}", e);
            AppError::InternalError
        })?;

    if user.is_none() {
        return Err(AppError::UserNotFound(user_id.to_string()));
    }

    return Ok(user.unwrap());
}
