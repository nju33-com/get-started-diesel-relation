use diesel::pg::PgConnection;
use diesel::prelude::*;
use get_started_diesel_relation::{Post, User};

const DATABASE_URL: &'static str = "postgres://postgres:@localhost:5432";

fn main() {
  let connection =
    PgConnection::establish(DATABASE_URL).expect(&format!("Error connecting to {}", DATABASE_URL));

  let user = User {
    id: 1,
    name: "foo".into(),
  };

  let post = Post::belonging_to(&user)
    .get_result::<Post>(&connection)
    .expect("Error in getting posts");
  println!("{:#?}", post);
}
