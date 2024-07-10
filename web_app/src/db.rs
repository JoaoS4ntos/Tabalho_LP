use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;

pub type DbPool = Pool<ConnectionManager<PgConnection>>; // Exemplo com Diesel e PostgreSQL

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


