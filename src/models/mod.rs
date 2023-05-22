use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use super::schema::posts;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct Post{
  pub id: i32,
  pub title: String,
  pub body: String,
  pub published: bool,
}