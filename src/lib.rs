#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate chrono;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate dotenv;

mod config;
mod controllers;
mod db;
mod errors;
mod models;
mod response;
mod routes;
mod schema;
mod validators;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use routes::auth;
use routes::post;
use std::env;

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();

    rocket::custom(config::setup::setup_with_env())
        .mount("/api", routes![post::create_post])
        .attach(db::connect::Conn::fairing())
}
