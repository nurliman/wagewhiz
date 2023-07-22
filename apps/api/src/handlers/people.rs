use crate::{errors::AppError, models, services};
use axum::{extract::Path, Json};

pub async fn get_people() -> Result<Json<Vec<models::Person>>, AppError> {
    let people = services::people::get_people().await?;
    Ok(Json(people))
}

pub async fn get_person_by_id(
    Path(person_id): Path<String>,
) -> Result<Json<models::Person>, AppError> {
    let person = services::people::get_person_by_id(&person_id).await?;
    Ok(Json(person))
}
