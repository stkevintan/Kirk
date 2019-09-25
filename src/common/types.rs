use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
  login: String,
  id: i32,
  avatar_url: String,
  url: String,
}

#[derive(Deserialize, Debug)]
pub struct Label {
  id: i32,
  url: String,
  name: String,
  color: String,
}

#[derive(Deserialize, Debug)]
pub struct Post {
  id: i32,
  url: String,
  number: i32,
  title: String,
  user: User,
  labels: Vec<Label>,
  created_at: String,
  updated_at: String,
  body: String,
}
