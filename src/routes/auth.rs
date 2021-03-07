use crate::validators;

use std::path::{Path, PathBuf};

// #[macro_use]
// extern crate rocket;

// use chrono::DateTime;
use rocket::request::{Form, FromForm};
use rocket::response::NamedFile;
use serde::Deserialize;
use validator::Validate;

use validators::basics::validate_country;

#[derive(FromForm, Deserialize, Validate, Debug)]
pub struct User {
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
pub fn index() -> &'static str {
    "Hello world!"
}

#[post("/signup", data = "<user>")]
pub fn create(user: Form<User>) {
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
