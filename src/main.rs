#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate chrono;

pub mod response;
pub mod validators;

use std::path::{Path, PathBuf};

// use chrono::DateTime;
use rocket::request::Form;
use rocket::response::NamedFile;
use validator::Validate;

use serde::Deserialize;

use validators::basics::validate_country;

#[derive(FromForm, Deserialize, Validate, Debug)]
struct User {
    #[validate(email)]
    email: String,
    phone: Option<String>,
    password: String,
    dob: String,
    #[validate(length(min = 3), custom = "validate_country")]
    country: String,
    state: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[post("/signup", data = "<user>")]
fn create(user: Form<User>) {
    println!("the new user's information {:#?}", user);
    // handle with
    // Content-Type: application/x-www-form-urlencoded
}

#[post("/login")]
fn login() {
    // handle login with
    // Content-Type: application/x-www-form-urlencoded
    // using _method: "get" on the frontend
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
