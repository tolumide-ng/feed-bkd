use crate::controllers::post;
use rocket_contrib::{json, json::JsonValue};

#[get("/")]
pub fn get_posts() -> JsonValue {
    let result = post::get_posts(5);

    json!({ "posts": result })
}
