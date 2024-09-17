use crate::{
    database::DatabaseProvider, entities::user_accounts, errors::AppError, types::AppResult,
};
use nject::injectable;
use sea_orm::*;
use uuid::Uuid;

#[injectable]
pub struct UsersRepository {
    db: DatabaseProvider,
}

impl UsersRepository {
    pub async fn find_by_username(&self, username: &str) -> AppResult<user_accounts::Model> {
        let conn = self.db.get_db_conn().await;
        let user = user_accounts::Entity::find()
            .filter(user_accounts::Column::Username.eq(username))
            .one(conn)
            .await
            .map_err(|e| {
                tracing::error!("Error getting user: {:?}", e);
                AppError::InternalError
            })?;
        user.ok_or_else(|| {
            AppError::ResourceNotFound(format!("User with username: {} not found", username))
        })
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> AppResult<user_accounts::Model> {
        let conn = self.db.get_db_conn().await;
        let user = user_accounts::Entity::find_by_id(user_id)
            .one(conn)
            .await
            .map_err(|e| {
                tracing::error!("Error getting user: {:?}", e);
                AppError::InternalError
            })?;
        user.ok_or_else(|| {
            AppError::ResourceNotFound(format!("User with id: {} not found", user_id))
        })
    }
}
