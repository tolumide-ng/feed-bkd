// use diesel::deserialize::{self, Queryable};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
