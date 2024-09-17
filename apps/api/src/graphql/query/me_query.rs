use crate::{entities::user_accounts, errors::AppError, graphql::auth::TokenVerifier};
use async_graphql::{Context, ErrorExtensions, Object, Result};

#[derive(Default)]
pub(super) struct MeQuery;

#[Object]
impl MeQuery {
    async fn me(&self, ctx: &Context<'_>) -> Result<user_accounts::Model> {
        let token_verifier = ctx.data::<TokenVerifier>().map_err(|e| {
            tracing::error!("Failed to get token verifier: {:?}", e);
            AppError::InternalError.extend()
        })?;
        token_verifier.get_verified_user(ctx).await
    }
}
