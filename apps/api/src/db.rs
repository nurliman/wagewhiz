use crate::{env, errors::AppError};
use bb8::{Pool, PooledConnection};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use std::time::Duration;
use tokio::sync::OnceCell;

pub type DbPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub type DbConn = PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>;

static DB: OnceCell<DbPool> = OnceCell::const_new();

async fn init_db() -> Result<DbPool, AppError> {
    let env_var = env::get_env_var()?;

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env_var.db_user, env_var.db_pass, env_var.db_host, env_var.db_port, env_var.db_name
    );

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = Pool::builder()
        .connection_timeout(Duration::from_secs(8))
        .build(config)
        .await
        .map_err(|e| {
            tracing::error!("Error creating pool: {:?}", e);
            AppError::DbError(e)
        })?;

    Ok(pool)
}

pub async fn get_pool() -> Result<&'static DbPool, AppError> {
    Ok(DB.get_or_init(|| async { init_db().await.unwrap() }).await)
}

pub async fn get_connection() -> Result<DbConn, AppError> {
    let pool = get_pool().await?;
    let conn = pool.get().await.map_err(|e| {
        tracing::error!("Error getting connection: {:?}", e);
        AppError::InternalError
    })?;

    Ok(conn)
}
