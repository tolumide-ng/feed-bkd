use diesel;
use monoli;

use self::diesel::prelude::*;
use self::models::posts::Post;
use self::monoli::*;
// use crate::models::posts::Post;

// pub struct Post {
//     title: String,
//     body: String,
// }

pub fn get_posts(limit: i64) -> Vec<Post> {
    use monoli::schema::posts::dsl::*;
    let connection = establish_connection();

    let results = posts
        .filter(published.eq(true))
        .limit(limit)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    results
}