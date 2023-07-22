use crate::{db, errors::AppError, models::Person, schema::people};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub async fn get_people() -> Result<Vec<Person>, AppError> {
    let mut conn = db::get_connection().await?;
    let people = people::table
        .select(Person::as_select())
        .limit(5)
        .load(&mut *conn)
        .await
        .map_err(|e| {
            tracing::error!("Error getting people: {:?}", e);
            AppError::InternalError
        })?;

    Ok(people)
}

pub async fn get_person_by_id(person_id: &str) -> Result<Person, AppError> {
    let mut conn = db::get_connection().await?;
    let person_uuid =
        Uuid::parse_str(person_id).map_err(|_| AppError::InvalidUuid(person_id.to_string()))?;
    let person = people::table
        .select(Person::as_select())
        .find(person_uuid)
        .first(&mut *conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => AppError::PersonNotFound(person_id.to_string()),
            _ => AppError::InternalError,
        })?;

    Ok(person)
}
