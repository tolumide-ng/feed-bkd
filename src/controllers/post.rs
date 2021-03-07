use monoli;

use crate::diesel::prelude::*;
use crate::models::posts::{NewPost, Post};
use crate::schema;
use monoli::establish_connection;

pub fn get_posts(limit: i64) -> Vec<Post> {
    use schema::posts::dsl::{posts, published};
    let connection = establish_connection();

    let results = posts
        .filter(published.eq(true))
        .limit(limit)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    results
}

// add a data guard to this later, should be only for authenticated users
pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str, publish: bool) -> Post {
    use schema::posts;
    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
