#[macro_use]
extern crate rocket;

#[macro_use]
extern crate chrono;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate dotenv;

#[macro_use]
extern crate rocket_contrib;

pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
