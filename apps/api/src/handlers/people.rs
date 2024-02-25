use crate::{entities::people, errors::AppError, services};
use axum::{extract::Path, Json};

pub async fn get_people() -> Result<Json<Vec<people::Model>>, AppError> {
    let people = services::people::get_people().await?;
    Ok(Json(people))
}

pub async fn get_person_by_id(
    Path(person_id): Path<String>,
) -> Result<Json<people::Model>, AppError> {
    let person = services::people::get_person_by_id(&person_id).await?;
    Ok(Json(person))
}
