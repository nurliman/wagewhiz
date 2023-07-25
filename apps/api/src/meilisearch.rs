use crate::{env, errors::AppError};
use meilisearch_sdk::Client;
use tokio::sync::OnceCell;

static MEILISEARCH_CLIENT: OnceCell<Client> = OnceCell::const_new();

async fn init_meilisearch_client() -> Result<Client, AppError> {
    let env_var = env::get_env_var()?.clone();

    let client = Client::new(
        env_var.meilisearch_url.clone(),
        Some(env_var.meilisearch_api_key.clone()),
    );

    Ok(client)
}

pub async fn get_client() -> Result<&'static Client, AppError> {
    Ok(MEILISEARCH_CLIENT
        .get_or_init(|| async { init_meilisearch_client().await.unwrap() })
        .await)
}
