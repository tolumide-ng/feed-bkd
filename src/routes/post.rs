use crate::controllers::post as posts;
use rocket_contrib::{json, json::JsonValue};

#[get("/posts")]
pub fn get_posts() -> JsonValue {
    let result = posts::get_posts(5);

    json!({ "posts": result })
}

#[post("/posts")]
pub fn create_post() {
    // let result = post::create_post(conn, title, body, publish);
    // json!({})
}
