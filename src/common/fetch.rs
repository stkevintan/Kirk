use super::types::*;
use failure::Error;
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
      token: concat!("81e7", "f49ac", "5d5b7", "20913d74a5b2f07cf6be93", "eaef"),
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
