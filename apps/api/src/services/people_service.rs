use crate::{
    entities::people::Model as Person,
    repositories::people::PeopleRepository,
    types::{AppResult, PaginationParams, PaginationResponse},
};
use nject::injectable;
use uuid::Uuid;

#[injectable]
pub struct PeopleService {
    people_repo: PeopleRepository,
}

impl PeopleService {
    pub async fn list(&self, params: PaginationParams) -> AppResult<PaginationResponse<Person>> {
        self.people_repo.list(params).await
    }

    pub async fn find_by_id(&self, person_id: Uuid) -> AppResult<Person> {
        self.people_repo.find_by_id(person_id).await
    }
}
