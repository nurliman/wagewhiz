use crate::{env, errors::AppError};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use std::time::Duration;
use tokio::sync::OnceCell;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

static DB: OnceCell<Pool> = OnceCell::const_new();

async fn init_db() -> Result<Pool, AppError> {
    let env_var = env::get_env_var()?.clone();

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env_var.db_user, env_var.db_pass, env_var.db_host, env_var.db_port, env_var.db_name
    );

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder()
        .connection_timeout(Duration::from_secs(8))
        .build(config)
        .await
        .map_err(|e| {
            tracing::error!("Error creating pool: {:?}", e);
            AppError::DbError(e)
        })?;

    Ok(pool)
}

pub async fn get_pool() -> Result<&'static Pool, AppError> {
    Ok(DB.get_or_init(|| async { init_db().await.unwrap() }).await)
}