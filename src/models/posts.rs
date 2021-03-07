// use diesel::deserialize::{self, Queryable};
use crate::config::setup::DATE_FORMAT;
use crate::schema::posts;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub slug: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub description: &'a str,
}

#[derive(Serialize)]
pub struct PostJson {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub slug: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}
