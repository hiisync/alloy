use core::panic;
use diesel::prelude::*;
use diesel::{Connection, PgConnection as DB};
use dotenvy::dotenv;
use std::env;

pub mod middleware;
pub mod models;
pub mod schema;

pub fn connect_db() -> DB {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    DB::establish(&database_url)
        .unwrap_or_else(|e: ConnectionError| panic!("Error connecting: {}", e))
}
