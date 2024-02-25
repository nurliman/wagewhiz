use crate::{db, entities::people, errors::AppError};
use sea_orm::*;
use uuid::Uuid;

pub async fn get_people() -> Result<Vec<people::Model>, AppError> {
    let conn = db::get_connection().await?;

    let people = people::Entity::find()
        .limit(5)
        .all(conn)
        .await
        .map_err(|e| {
            tracing::error!("Error getting people: {:?}", e);
            AppError::InternalError
        })?;

    Ok(people)
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
