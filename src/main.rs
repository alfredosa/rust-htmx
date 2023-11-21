mod api;
mod logging;
mod models;
mod schema;

use actix_web::{web, App, HttpServer};
use api::index::index;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::env;

pub fn connect_db() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let _database = connect_db();

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
