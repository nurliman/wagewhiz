use crate::{env, errors::AppError};
use sea_orm::{ConnectOptions, Database, DbConn};
use std::time::Duration;
use tokio::sync::OnceCell;

static DB: OnceCell<DbConn> = OnceCell::const_new();

async fn init_db() -> Result<DbConn, AppError> {
    let env_var = env::get_env_var()?;

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env_var.db_user, env_var.db_pass, env_var.db_host, env_var.db_port, env_var.db_name
    );

    let mut opt = ConnectOptions::new(db_url);

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let db = Database::connect(opt).await.map_err(|e| {
        tracing::error!("Error connecting to db: {:?}", e);
        AppError::DbError(e)
    })?;

    Ok(db)
}

// result reference of DbConn
pub async fn get_connection() -> Result<&'static DbConn, AppError> {
    let conn = DB.get_or_init(|| async { init_db().await.unwrap() }).await;

    Ok(conn)
}
