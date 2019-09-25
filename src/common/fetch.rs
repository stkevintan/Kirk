use super::types::*;
use failure::Error;
use serde_json::json;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::Callback;

pub struct CustomFetchService {
  token: &'static str,
  fetch_service: FetchService,
}

impl Default for CustomFetchService {
  fn default() -> Self {
    Self {
      token: "c10661fa4e6f5e4fda03e0b7568628428ce3c155",
      fetch_service: FetchService::default(),
    }
  }
}

impl CustomFetchService {
  pub fn new(token: &'static str) -> Self {
    Self {
      token,
      fetch_service: FetchService::new(),
    }
  }

  #[allow(dead_code)]
  pub fn fetch_posts(
    &mut self,
    callback: Callback<Response<Json<Result<Vec<Post>, Error>>>>,
  ) -> FetchTask {
    let request = Request::get("https://api.github.com/repos/stkevintan/kirk-blog/issues")
      .header("Authorization", &format!("token {}", self.token))
      .body(Nothing)
      .expect("fetch posts failed");

    self.fetch_service.fetch(request, callback)
  }
}
