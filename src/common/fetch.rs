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
      token: concat!("938367078c8b020c06ec486e636d1b471e2f", "9970"),
      fetch_service: FetchService::default(),
    }
  }
}

impl CustomFetchService {
  // pub fn new(token: &'static str) -> Self {
  //   Self {
  //     token,
  //     fetch_service: FetchService::new(),
  //   }
  // }

  #[allow(dead_code)]
  pub fn fetch_posts(
    &mut self,
    pagination: &Pagination,
    callback: Callback<Response<Json<Result<Vec<Post>, Error>>>>,
  ) -> FetchTask {
    let path = "https://api.github.com/repos/stkevintan/kirk-blog/issues";
    let url = format!(
      "{}?labels={}&page={}&per_page={}",
      path, "Blog", pagination.current, pagination.per_page
    );

    let request = Request::get(url)
      .header("Authorization", &format!("token {}", self.token))
      .body(Nothing)
      .expect("fetch posts failed");

    self.fetch_service.fetch(request, callback)
  }

  #[allow(dead_code)]
  pub fn fetch_post(
    &mut self,
    id: u32,
    callback: Callback<Response<Json<Result<Post, Error>>>>,
  ) -> FetchTask {
    let request = Request::get(format!(
      "https://api.github.com/repos/stkevintan/kirk-blog/issues/{}",
      id
    ))
    .header("Authorization", &format!("token {}", self.token))
    .body(Nothing)
    .expect("fetch post failed");

    self.fetch_service.fetch(request, callback)
  }

  #[allow(dead_code)]
  pub fn fetch_about(
    &mut self,
    callback: Callback<Response<Json<Result<Vec<Post>, Error>>>>,
  ) -> FetchTask {
    let request = Request::get(format!(
      "https://api.github.com/repos/stkevintan/kirk-blog/issues?labels=About&per_page=1&page=1",
    ))
    .header("Authorization", &format!("token {}", self.token))
    .body(Nothing)
    .expect("fetch about failed");

    self.fetch_service.fetch(request, callback)
  }

  #[allow(dead_code)]
  pub fn fetch_comments(
    &mut self,
    id: u32,
    callback: Callback<Response<Json<Result<Vec<Comment>, Error>>>>,
  ) -> FetchTask {
    let request = Request::get(format!(
      "https://api.github.com/repos/stkevintan/kirk-blog/issues/{}/comments",
      id
    ))
    .header("Authorization", &format!("token {}", self.token))
    .body(Nothing)
    .expect("fetch comments failed");

    self.fetch_service.fetch(request, callback)
  }
}
