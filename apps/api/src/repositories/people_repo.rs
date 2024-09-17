use crate::{
    constants::{DEFAULT_MAX_PAGE_SIZE, DEFAULT_PAGE, DEFAULT_PAGE_SIZE},
    database::DatabaseProvider,
    entities::people,
    errors::AppError,
    types::{AppResult, PaginationParams, PaginationResponse},
};
use nject::injectable;
use sea_orm::{EntityTrait, PaginatorTrait};
use uuid::Uuid;

#[injectable]
pub struct PeopleRepository {
    db: DatabaseProvider,
}

impl PeopleRepository {
    pub async fn list(
        &self,
        params: PaginationParams,
    ) -> AppResult<PaginationResponse<people::Model>> {
        let page = params.page.unwrap_or(DEFAULT_PAGE).max(1);
        let page_size = params
            .page_size
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .clamp(1, DEFAULT_MAX_PAGE_SIZE);

        let conn = self.db.get_db_conn().await;
        let people_pages = people::Entity::find().paginate(conn, page_size);

        let (items, total) = tokio::join!(
            async { people_pages.fetch_page(page - 1).await.unwrap_or_default() },
            async { people_pages.num_items().await.unwrap_or_default() },
        );

        Ok(PaginationResponse {
            items,
            page,
            page_size,
            total,
        })
    }

    pub async fn find_by_id(&self, person_id: Uuid) -> AppResult<people::Model> {
        let conn = self.db.get_db_conn().await;
        let person = people::Entity::find_by_id(person_id)
            .one(conn)
            .await
            .map_err(|e| {
                tracing::error!("Error getting person: {:?}", e);
                AppError::InternalError
            })?;

        person.ok_or_else(|| {
            AppError::ResourceNotFound(format!("Person with id: {} not found", person_id))
        })
    }
}
