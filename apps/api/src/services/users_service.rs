use crate::{entities::user_accounts, repositories::users::UsersRepository, types::AppResult};
use nject::injectable;
use uuid::Uuid;

#[injectable]
pub struct UsersService {
    users_repo: UsersRepository,
}

impl UsersService {
    pub async fn find_by_username(&self, username: &str) -> AppResult<user_accounts::Model> {
        self.users_repo.find_by_username(username).await
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> AppResult<user_accounts::Model> {
        self.users_repo.find_by_id(user_id).await
    }
}
