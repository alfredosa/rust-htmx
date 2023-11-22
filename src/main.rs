#[macro_use]
extern crate validator_derive;

mod config;
mod handlers;
mod models;

use actix_web::{middleware::Logger, web, App, HttpServer};
use color_eyre::Result;
use config::*;
use handlers::{app_config, index::index};

use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server Configuration");

    let pool = config.db_pool().await?;
    let crypto_service = config.crypto_service();

    info!("Starting Server at http://{}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(pool.clone())
            .app_data(crypto_service.clone())
            .configure(app_config)
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
