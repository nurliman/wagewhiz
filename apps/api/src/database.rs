use crate::{config::ConfigProvider, errors::AppError};
use migration::{Migrator, MigratorTrait};
use nject::injectable;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tokio::sync::OnceCell;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

#[injectable]
pub struct DatabaseProvider {
    config_provider: ConfigProvider,
}

impl DatabaseProvider {
    pub async fn get_db_conn(&self) -> &'static DatabaseConnection {
        DB.get_or_init(|| self.initialize()).await
    }

    async fn initialize(&self) -> DatabaseConnection {
        let config = self.config_provider.get_config();
        let db_url = config.get_database_url();
        let opt = self.connection_options(&db_url);

        Database::connect(opt)
            .await
            .expect("Failed to connect to database")
    }

    fn connection_options(&self, db_url: &str) -> ConnectOptions {
        let mut opt = ConnectOptions::new(db_url.to_owned());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8));
        opt
    }
}

pub async fn migrate(conn: &DatabaseConnection) -> Result<(), AppError> {
    Migrator::up(conn, None).await.map_err(|e| {
        tracing::error!("Database migration error: {:?}", e);
        AppError::InternalError
    })
}
