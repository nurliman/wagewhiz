use crate::{entities::user_accounts, errors::AppError, services::users::UsersService, Provider};
use api::authenticated;
use async_graphql::{Context, ErrorExtensions, Object, Result};
use uuid::Uuid;

#[derive(Default)]
pub(super) struct UsersQuery;

#[Object]
impl UsersQuery {
    #[authenticated]
    async fn user(&self, ctx: &Context<'_>, id: Uuid) -> Result<user_accounts::Model> {
        let provider = ctx.data::<Provider>().map_err(|e| {
            tracing::error!("Failed to get provider: {:?}", e);
            AppError::InternalError.extend()
        })?;
        let service = provider.provide::<UsersService>();
        let user = service.find_by_id(id).await.map_err(|e| e.extend())?;
        Ok(user)
    }
}
