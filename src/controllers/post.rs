use crate::db;
use crate::diesel::prelude::*;
use crate::models::posts::{NewPost, Post};
use crate::schema;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Debug, Deserialize)]
pub struct NewPostData {
    #[validate(length(min = 1))]
    title: Option<String>,
    // description: Option<String>,
    #[validate(length(min = 1))]
    body: Option<String>,
    #[validate(length(min = 1))]
    description: Option<String>,
}

#[derive(Deserialize)]
pub struct NewCreatePost {
    post: NewPostData,
}

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
pub fn create_post<'a>(new_post: Json<NewCreatePost>, conn: &PgConnection) -> Post {
    use schema::posts;
    let new_post = NewPost {
        title: title,
        body: body,
        description,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
