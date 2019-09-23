use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct API<T> {
  pub code: i32,
  pub data: Option<T>,
  pub msg: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PageResult<T> {
  pub total: i32,
  pub items: Option<Vec<T>>,
}
