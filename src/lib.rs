#[macro_use]
extern crate diesel;

pub mod schema;

use schema::{posts, users};

#[derive(Debug, Queryable, Identifiable)]
pub struct User {
  pub id: i32,
  pub name: String,
}

#[derive(Debug, Queryable, Associations, Identifiable)]
#[belongs_to(User, foreign_key = "userId")]
pub struct Post {
  pub id: i32,
  pub title: String,
  #[column_name = "userId"]
  pub user_id: i32,
}
