use crate::{db, errors, models, schema};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub async fn get_user_by_username(username: &str) -> Result<models::User, errors::AppError> {
    let pool = db::get_pool().await.map_err(|e| {
        tracing::error!("Error getting pool: {:?}", e);
        errors::AppError::InternalError
    })?;
    let mut conn = pool.get().await.map_err(|e| {
        tracing::error!("Error getting connection: {:?}", e);
        errors::AppError::InternalError
    })?;
    let res_user = schema::users::table
        .select(models::User::as_select())
        .filter(schema::users::username.eq(username))
        .first(&mut *conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                errors::AppError::UsernameNotFound(username.to_string())
            }
            _ => errors::AppError::InternalError,
        })?;

    return Ok(res_user);
}

pub async fn get_user_by_id(user_id: &str) -> Result<models::User, errors::AppError> {
    let pool = db::get_pool().await?;
    let mut conn = pool.get().await.map_err(|e| {
        tracing::error!("Error getting connection: {:?}", e);
        errors::AppError::InternalError
    })?;
    let user_uuid = Uuid::parse_str(user_id).map_err(|e| {
        tracing::error!("Error parsing uuid: {:?}", e);
        errors::AppError::InvalidUuid(user_id.to_string())
    })?;
    let res_user = schema::users::table
        .select(models::User::as_select())
        .find(user_uuid)
        .first(&mut *conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => errors::AppError::UserNotFound(user_id.to_string()),
            _ => errors::AppError::InternalError,
        })?;

    return Ok(res_user);
}
