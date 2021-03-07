use monoli;

use self::monoli::*;
// use crate::models::posts::Post;

use self::models::posts::Post;
use crate::controllers::post;
use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_contrib::{json, json::JsonValue};
use std::io::Cursor;

fn to_json(post: Post) -> JsonValue {
    json!({ "data": post })
}

#[get("/")]
pub fn get_posts() -> JsonValue {
    let result = post::get_posts(5);

    json!({ "posts": result })
}
