extern crate rocket;
use rocket::{routes, launch};

mod models;
mod services;
pub mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![services::create_post])
        .mount("/", routes![services::list])
}
