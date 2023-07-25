use crate::{
    errors::{AppError, Query},
    models::JobTitle,
    services, utils,
};
use axum::Json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SearchJobTitlesParams {
    #[serde(
        default,
        deserialize_with = "utils::serde::string::empty_string_as_none"
    )]
    pub q: Option<String>,
    pub limit: Option<usize>,
}

pub async fn search_job_titles(
    Query(params): Query<SearchJobTitlesParams>,
) -> Result<Json<Vec<JobTitle>>, AppError> {
    let search_query = params.q.unwrap_or_default();
    let job_titles = services::misc::search_job_titles(&search_query, params.limit).await?;

    Ok(Json(job_titles))
}
