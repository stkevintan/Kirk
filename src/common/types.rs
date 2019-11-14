use serde_derive::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct User {
  pub login: String,
  pub id: i32,
  pub avatar_url: String,
  pub url: String,
  pub html_url: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Label {
  pub id: i32,
  pub url: String,
  pub name: String,
  pub color: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Post {
  pub id: i32,
  pub url: String,
  pub number: i32,
  pub title: String,
  pub user: User,
  pub labels: Vec<Label>,
  pub created_at: String,
  pub updated_at: String,
  pub body: String,
}
