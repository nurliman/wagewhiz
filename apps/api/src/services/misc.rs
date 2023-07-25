use crate::{errors::AppError, meilisearch, models::JobTitle};

pub static DEFAULT_JOB_TITLES_LIMIT: usize = 5;
pub static MAX_JOB_TITLES_LIMIT: usize = 20;

pub async fn search_job_titles(
    query: &str,
    limit: Option<usize>,
) -> Result<Vec<JobTitle>, AppError> {
    let client = meilisearch::get_client().await?;
    let limit = std::cmp::min(limit.unwrap_or(DEFAULT_JOB_TITLES_LIMIT), MAX_JOB_TITLES_LIMIT);
    let index = client.index("job-titles");
    let search_results = index
        .search()
        .with_query(query)
        .with_limit(limit)
        .execute::<JobTitle>()
        .await
        .map_err(|e| {
            tracing::error!("Error searching job titles: {:?}", e);
            AppError::InternalError
        })?;

    let mut job_titles = Vec::new();
    for hit in search_results.hits {
        job_titles.push(hit.result);
    }

    Ok(job_titles)
}
