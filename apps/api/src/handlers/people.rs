use crate::{errors::AppError, models, services};
use axum::Json;

pub async fn get_people() -> Result<Json<Vec<models::Person>>, AppError> {
    let people = services::people::get_people().await?;
    Ok(Json(people))
}
