mod handlers;
mod config;
mod logging;
mod models;

use actix_web::{web, App, HttpServer, middleware::Logger};
use handlers::{index::index, app_config};
use color_eyre::Result;
use config::*;

use tracing::info;

// pub fn connect_db() -> PgConnection {
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server Configuration");

    info!("Starting Server at http://{}:{}", config.host, config.port);

    HttpServer::new(move || App::new()
        .wrap(Logger::default())
        .configure(app_config)
        .service(web::resource("/").route(web::get().to(index))))
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;

    Ok(())
}
