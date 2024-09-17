use crate::{
    entities::people,
    errors::AppError,
    services::people::PeopleService,
    types::{PaginationParams, PaginationResponse},
    Provider,
};
use api::authenticated;
use async_graphql::{Context, ErrorExtensions, Object, Result};
use uuid::Uuid;

#[derive(Default)]
pub(super) struct PeopleQuery;

#[Object]
impl PeopleQuery {
    #[authenticated]
    async fn people(
        &self,
        ctx: &Context<'_>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<PaginationResponse<people::Model>> {
        let provider = ctx.data::<Provider>().map_err(|e| {
            tracing::error!("Failed to get provider: {:?}", e);
            AppError::InternalError.extend()
        })?;

        let people_service = provider.provide::<PeopleService>();
        let people = people_service
            .list(PaginationParams { page, page_size })
            .await
            .map_err(|e| e.extend())?;
        Ok(people)
    }

    #[authenticated]
    async fn person(&self, ctx: &Context<'_>, id: Uuid) -> Result<people::Model> {
        let provider = ctx.data::<Provider>().map_err(|e| {
            tracing::error!("Failed to get provider: {:?}", e);
            AppError::InternalError.extend()
        })?;

        let people_service = provider.provide::<PeopleService>();
        let person = people_service
            .find_by_id(id)
            .await
            .map_err(|e| e.extend())?;
        Ok(person)
    }
}
