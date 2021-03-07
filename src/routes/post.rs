use crate::{
    controllers::post::{self as posts, NewCreatePost},
    errors::field_validator::FieldValidator,
};
use crate::{db, errors::field_validator::Errors};
use rocket_contrib::json::{self, Json, JsonValue};

// #[get("/posts")]
// pub fn get_posts() -> JsonValue {
//     let result = posts::get_posts(5);

//     json!({ "posts": result })
// }
pub struct Auth {}

#[post("/posts", format = "json", data = "<new_post>")]
pub fn create_post(
    new_post: Json<NewCreatePost>,
    conn: db::connect::Conn,
) -> Result<JsonValue, Errors> {
    let new_post = new_post.into_inner().post;

    let mut extractor = FieldValidator::validate(&new_post);
    let title = extractor.extract("title", new_post.title);
    let description = extractor.extract("description", new_post.description);
    let body = extractor.extract("body", new_post.body);

    extractor.check()?;
}
