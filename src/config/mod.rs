use color_eyre::Result;
use dotenvy::dotenv;
use serde::Deserialize;
use tracing::{info, instrument};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt::init();

        info!("Loading configuration");

        let mut cfg =
            config::Config::builder().add_source(config::Environment::default().try_parsing(true));

        match cfg.build() {
            Ok(config) => {
                let host = config.get_string("host")?;
                let port = config.get_int("port")?.try_into()?;
                Ok(Config { host, port })
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
