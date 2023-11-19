mod models;
mod schema;
mod logging;

use std::env;
use logging::log_printer;
use diesel::{PgConnection, Connection};
use dotenvy::dotenv;



pub fn connect_db() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut database = connect_db();

    log_printer(
        "info",
        "Connected to the DB successfully!",
        line!(),
        file!(),
    );
    
    Ok(())
}
