use crate::errors::field_validator::{Errors, FieldErrorCode, FieldValidator};
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Debug, Deserialize)]
pub struct NewPostData {
    #[validate(length(min = 1))]
    pub title: Option<String>,
    // description: Option<String>,
    #[validate(length(min = 1))]
    pub body: Option<String>,
    #[validate(length(min = 1))]
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct NewCreatePost {
    pub post: NewPostData,
}

pub fn create_posts<'a>(new_post: Json<NewCreatePost>) {
    let new_post = new_post.into_inner().post;

    let mut extractor = FieldValidator::validate(&new_post);
    let title = extractor.extract("title", new_post.title).as_str();
    let description = extractor
        .extract("description", new_post.description)
        .as_str();
    let body = extractor.extract("body", new_post.body).as_str();
}
