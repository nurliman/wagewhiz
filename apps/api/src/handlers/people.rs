use crate::{
    entities::people,
    errors::AppError,
    services::{self, people::GetPeople},
    types::{PaginationParams, PaginationResponse},
};
use axum::{
    extract::{Path, Query},
    Json,
};
use std::cmp::{max, min};

pub async fn get_people(
    Query(query): Query<PaginationParams>,
) -> Result<Json<PaginationResponse<people::Model>>, AppError> {
    let page = query.page.unwrap_or(1);
    let page = max(page, 1);

    let page_size = query.page_size.unwrap_or(8);
    let page_size: u64 = min(page_size, 50);

    let people = GetPeople::new().page(page).page_size(page_size);
    let (data, total) = tokio::join!(people.data(), people.count());
    let data = data?;
    let total = total?;

    Ok(Json(PaginationResponse {
        data,
        page,
        page_size,
        total,
    }))
}

pub async fn get_person_by_id(
    Path(person_id): Path<String>,
) -> Result<Json<people::Model>, AppError> {
    let person = services::people::get_person_by_id(&person_id).await?;
    Ok(Json(person))
}
