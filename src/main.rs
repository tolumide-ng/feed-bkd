#![feature(proc_macro_hygiene, decl_macro)]
// embed_migrations!("../migrations/sqlite");

pub mod controllers;
pub mod models;
pub mod response;
pub mod routes;
pub mod validators;

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate diesel;
extern crate dotenv;

use routes::auth;
use routes::post;

fn main() {
    rocket::ignite()
        .mount("/", routes![auth::index, auth::create, post::get_posts])
        .launch();
}
