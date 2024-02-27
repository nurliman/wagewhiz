use crate::{db, entities::people, errors::AppError};
use sea_orm::*;
use uuid::Uuid;

pub struct GetPeople {
    page: Option<u64>,
    page_size: Option<u64>,
}

impl GetPeople {
    pub fn new() -> Self {
        Self {
            page: None,
            page_size: None,
        }
    }

    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }

    pub fn page_size(mut self, page_size: u64) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub async fn data(&self) -> Result<Vec<people::Model>, AppError> {
        let conn = db::get_connection().await?;
        let mut query = people::Entity::find();

        if let (Some(page), Some(page_size)) = (self.page, self.page_size) {
            let offset = (page - 1) * page_size;
            query = query.offset(offset).limit(page_size);
        }

        let people = query.all(conn).await.map_err(|e| {
            tracing::error!("Error getting people: {:?}", e);
            AppError::InternalError
        })?;

        Ok(people)
    }

    pub async fn count(&self) -> Result<u64, AppError> {
        let conn = db::get_connection().await?;
        let count = people::Entity::find().count(conn).await.map_err(|e| {
            tracing::error!("Error counting people: {:?}", e);
            AppError::InternalError
        })?;

        Ok(count)
    }
}

pub async fn get_person_by_id(person_id: &str) -> Result<people::Model, AppError> {
    let conn = db::get_connection().await?;

    let person_uuid =
        Uuid::parse_str(person_id).map_err(|_| AppError::InvalidUuid(person_id.to_string()))?;

    let person = people::Entity::find_by_id(person_uuid)
        .one(conn)
        .await
        .map_err(|e| {
            tracing::error!("Error getting person: {:?}", e);
            AppError::InternalError
        })?;

    if person.is_none() {
        return Err(AppError::PersonNotFound(person_id.to_string()));
    }

    return Ok(person.unwrap());
}
