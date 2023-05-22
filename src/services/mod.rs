// extern crate diesel;
// extern crate rocket;

use diesel::{PgConnection, Connection, RunQueryDsl};
use dotenvy::dotenv;
use rocket::response::Debug;
use rocket::response::status::Created;
use rocket::{post, serde::json::Json};
use serde::{Serialize, Deserialize};
use std::env;

use crate::schema;
use crate::models::Post;


pub fn establish_connection_pg() -> PgConnection {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Deserialize)]
pub struct NewPost {
  title: String,
  body: String,
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

// POST
#[post("/post", format = "json", data = "<post>")]
pub fn create_post(post: Json<NewPost>) -> Result<Created<Json<NewPost>>> {
  let connection = &mut establish_connection_pg();

  let new_post = Post {
    id: 1,
    title: post.title.to_string(),
    body: post.body.to_string(),
    published: true,
  };

  diesel::insert_into(self::schema::posts::dsl::posts)
    .values(&new_post)
    .execute(connection)
    .expect("Error saving new post");

  // set the response to 201
  Ok(Created::new("/").body(post))
}

// GET