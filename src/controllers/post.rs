use crate::db;
use crate::diesel::prelude::*;
use crate::errors::field_validator::{Errors, FieldErrorCode, FieldValidator};
use crate::models::posts::{NewPost, Post};
use crate::schema;
use diesel::pg::PgConnection;

// pub fn get_posts(limit: i64) -> Vec<Post> {
//     use schema::posts::dsl::{posts, published};

//     let results = posts
//         .filter(published.eq(true))
//         .limit(limit)
//         .load::<Post>(&connection)
//         .expect("Error loading posts");

//     results
// }

// add a data guard to this later, should be only for authenticated users
pub fn create_post<'a>(new_post: Json<NewCreatePost>) -> Result<JsonValue, Errors> {
    use schema::posts;

    let new_post = new_post.into_inner().post;

    let mut extractor = FieldValidator::validate(&new_post);
    let title = extractor.extract("title", new_post.title).as_str();
    let description = extractor
        .extract("description", new_post.description)
        .as_str();
    let body = extractor.extract("body", new_post.body).as_str();

    let new_post = &NewPost {
        title,
        body,
        description,
    };

    extractor.check()?;

    diesel::insert_into(posts::table)
        .values(new_post)
        .get_result::<Post>(conn)
        .expect("Error creating post");

    let post = {};

    Ok(json!({ "post": post }))
}
