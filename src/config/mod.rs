pub mod crypto;

use color_eyre::Result;
use dotenvy::dotenv;
use eyre::Context;
use eyre::WrapErr;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use tracing::{info, instrument, error};
use std::sync::Arc;
use self::crypto::CryptoService;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt::init();

        info!("Loading configuration");

        let cfg =
            config::Config::builder().add_source(config::Environment::default().try_parsing(true));

        match cfg.build() {
            Ok(config) => {
                let host = config.get_string("host")?;
                let port = config.get_int("port")?.try_into()?;
                let database_url = config.get_string("database_url")?;
                let secret_key = config.get_string("secret_key")?;
                Ok(Config {
                    host,
                    port,
                    database_url,
                    secret_key
                })
            }
            Err(e) => {
                Err(e.into())
            }
        }
    }

    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating database pool");

        PgPool::connect(&self.database_url)
            .await
            .context("Creating dtabase Connection pool")
    }

    pub fn crypto_service(&self) -> CryptoService {
        CryptoService { key: Arc::new(self.secret_key.clone()) }
    }
}
