use crate::{db, errors::AppError, models::UserAccount, schema};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub async fn get_user_by_username(username: &str) -> Result<UserAccount, AppError> {
    let mut conn = db::get_connection().await?;
    let user = schema::user_accounts::table
        .select(UserAccount::as_select())
        .filter(schema::user_accounts::username.eq(username))
        .first(&mut *conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => AppError::UsernameNotFound(username.to_string()),
            _ => AppError::InternalError,
        })?;

    return Ok(user);
}

pub async fn get_user_by_id(user_id: &str) -> Result<UserAccount, AppError> {
    let mut conn = db::get_connection().await?;
    let user_uuid =
        Uuid::parse_str(user_id).map_err(|_| AppError::InvalidUuid(user_id.to_string()))?;
    let user = schema::user_accounts::table
        .select(UserAccount::as_select())
        .find(user_uuid)
        .first(&mut *conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => AppError::UserNotFound(user_id.to_string()),
            _ => AppError::InternalError,
        })?;

    return Ok(user);
}
