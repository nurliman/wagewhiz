use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct EnvironmentVariables {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: String,

    pub db_name: String,

    pub db_user: String,

    pub db_pass: String,

    #[serde(default = "default_host")]
    pub db_host: String,

    #[serde(default = "default_db_port")]
    pub db_port: String,

    #[serde(default = "default_db_migration")]
    pub db_migration: bool,

    pub access_token_secret_key: String,

    pub access_token_public_key: String,

    pub refresh_token_secret_key: String,

    pub refresh_token_public_key: String,
}

fn default_host() -> String {
    String::from("127.0.0.1")
}

fn default_port() -> String {
    String::from("3001")
}

fn default_db_port() -> String {
    String::from("5432")
}

fn default_db_migration() -> bool {
    false
}

static ENV_VAR: Lazy<Result<EnvironmentVariables, AppError>> = Lazy::new(|| {
    dotenv().ok();
    envy::from_env::<EnvironmentVariables>().map_err(|e| {
        tracing::error!("Error parsing env variable: {:?}", e);
        AppError::EnvError(e)
    })
});

pub fn get_env_var() -> Result<&'static EnvironmentVariables, AppError> {
    Ok(ENV_VAR.as_ref().unwrap())
}
