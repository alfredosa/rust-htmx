mod api;
mod logging;
mod models;
mod schema;

use actix_files as fs;
use actix_web::{dev::ServiceRequest, web, App, HttpMessage, HttpResponse, HttpServer};
use api::index::index;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use logging::log_printer;
use std::env;
use tera::{Context, Tera};

pub fn connect_db() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut database = connect_db();

    log_printer(
        "info",
        "Connected to the DB successfully!",
        line!(),
        file!(),
    );

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .service(web::resource("/").route(web::get().to(index)))
        // .default_service(actix_web::web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
