use monoli;

use self::monoli::*;
use crate::controllers::post;
// use crate::models::posts::Post;
use self::models::posts::Post;

// use rocket::data::{self, Data, FromData};
// use rocket::http::Status;

// struct PostType {
//     limit: i64,
//     offset: u16,
// }

// // #[rocket::async_trait]
// impl FromData for PostType {
//     // use Error::*;

//     type Error = String;

//     fn from_data(request: &PostType, data: Data) -> data::Outcome<Self, Self::Error> {
//         // let a = request.limit;
//         use rocket::outcome::Outcome::*;

//         let limit: i64 = match request.limit.parse() {
//             Ok(age) => age,
//             Err(_) => return Failure((Status::Unpr)),
//         };

//         Success()
//     }
// }

// use std::io::Cursor;

// use rocket::http::ContentType;
// use rocket::request::Request;
// use rocket::response::{self, Responder, Response};

// impl<'r> Responder<'r, 'static> for Post {
//     fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
//         let person_string = format!("{}:{}", self.name, self.age);
//         Response::build()
//             .sized_body(person_string.len(), Cursor::new(person_string))
//             .raw_header("X-Person-Name", self.name)
//             .raw_header("X-Person-Age", self.age.to_string())
//             .header(ContentType::new("application", "x-person"))
//             .ok()
//     }
// }

#[get("/")]
pub fn get_posts() -> Vec<Post> {
    return post::get_posts(5);
}
