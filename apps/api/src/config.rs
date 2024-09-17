use crate::utils::serde_bool::deserialize_permissive_bool;
use dotenvy::dotenv;
use nject::injectable;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde_inline_default("127.0.0.1".to_string())]
    pub host: String,

    #[serde_inline_default(3001)]
    pub port: u16,

    pub db_name: String,

    pub db_user: String,

    pub db_pass: String,

    #[serde_inline_default("127.0.0.1".to_string())]
    pub db_host: String,

    #[serde_inline_default("5432".to_string())]
    pub db_port: String,

    #[serde(default, deserialize_with = "deserialize_permissive_bool")]
    pub db_migration: bool,

    pub access_token_secret_key: String,

    pub access_token_public_key: String,

    pub refresh_token_secret_key: String,

    pub refresh_token_public_key: String,
}

impl Config {
    pub fn get_database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, self.db_pass, self.db_host, self.db_port, self.db_name
        )
    }
}

static CONFIG: OnceCell<Config> = OnceCell::new();

#[injectable]
pub struct ConfigProvider;

impl ConfigProvider {
    pub fn get_config(&self) -> &'static Config {
        CONFIG.get_or_init(|| {
            dotenv().ok();
            envy::from_env().expect("Failed to load configuration")
        })
    }
}
