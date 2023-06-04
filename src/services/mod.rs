use diesel::{PgConnection, Connection, RunQueryDsl};
use dotenvy::dotenv;
use rocket::response::Debug;
use rocket::response::status::Created;
use rocket::{post, get, serde::json::Json};
use serde::{Serialize, Deserialize};
use std::env;

use crate::schema::{posts};
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

#[derive(Serialize, Deserialize)]
struct PostData {
  id: i32,
  title: String,
  body: String,
  published: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PostResponse {
  posts: Vec<PostData>
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

// POST
#[post("/post", format = "json", data = "<post>")]
pub fn create_post(post: Json<NewPost>) -> Result<Created<Json<NewPost>>> {
  let connection = &mut establish_connection_pg();

  // Get count of posts (to determine the next post ID)
  let results = posts::dsl::posts.load::<Post>(connection)
  .expect("Error loading posts");
  let total_posts = results.len() as i32;

  let new_post = Post {
    id: total_posts + 1,
    title: post.title.to_string(),
    body: post.body.to_string(),
    published: true,
  };

  diesel::insert_into(posts::dsl::posts)
    .values(&new_post)
    .execute(connection)
    .expect("Error saving new post");

  // set the response to 201
  Ok(Created::new("/").body(post))
}

#[get("/posts")]
pub fn list() -> Json<PostResponse> {
  let connection = &mut establish_connection_pg();
  let results = posts::dsl::posts
    .load::<Post>(connection)
    .expect("Error loading posts");

    let mut response_body = PostResponse {
      posts: Vec::new(),
    };

    for post in results.iter() {
      let post_data = PostData {
        id: post.id,
        title: post.title.to_string(),
        body: post.body.to_string(),
        published: post.published
      };

      response_body.posts.push(post_data);
    }

    Json(response_body)
}