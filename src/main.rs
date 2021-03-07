#![feature(proc_macro_hygiene, decl_macro)]
// embed_migrations!("../migrations/sqlite");

pub mod controllers;
pub mod models;
pub mod response;
pub mod routes;
pub mod schema;
pub mod validators;

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

use routes::auth;
use routes::post;

fn main() {
    rocket::ignite()
        .mount("/", routes![auth::index, auth::create, post::get_posts])
        .launch();
}
