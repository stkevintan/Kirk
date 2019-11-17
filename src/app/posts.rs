use crate::common::*;
use crate::components::Errors;
use crate::components::Loading;
use crate::components::Pagination;
use crate::components::PostItem;
#[warn(unused_must_use)]
use failure::Error;
use log::*;
use stdweb::{js, unstable::TryInto};
use yew::format::Json;
use yew::services::fetch::{FetchTask, Response};
use yew::services::Task;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Posts {
  link: ComponentLink<Posts>,
  fetch_service: CustomFetchService,
  state: State,
  error: Option<String>,
  pagination: types::Pagination,
}

#[derive(Default)]
pub struct State {
  posts: Vec<types::Post>,
  ft: Option<FetchTask>,
}

// impl std::fmt::Debug for State {
//   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//     writeln!(f, "Posts: {:?}", self.posts)?;
//     writeln!(
//       f,
//       "per_page {} page {}, last_page {}",
//       self.per_page, self.page, self.last_page
//     )
//   }
// }

pub enum Msg {
  FetchPosts,
  FetchReady(Vec<types::Post>),
  SetPagination(types::Pagination),
  Error(String),
}

impl Component for Posts {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Posts {
      link,
      fetch_service: CustomFetchService::default(),
      state: State::default(),
      error: None,
      pagination: types::Pagination::default(),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::FetchPosts => {
        let callback = self.link.send_back_batch(
          move |response: Response<Json<Result<Vec<types::Post>, Error>>>| {
            let mut ret = Vec::new();
            let (header, Json(body)) = response.into_parts();
            if let Err(err) = &body {
              trace!("error: {:?}", err);
            }
            if !header.status.is_success() {
              // TODO: get some hint
              ret.push(Msg::Error(format!(
                "Request failed with status: {}",
                header.status.as_str()
              )));
              return ret;
            }

            let headers = &header.headers;

            if let Some(page_link) = headers.get("Link").and_then(|value| value.to_str().ok()) {
              trace!("pagination: {:?}", page_link);
              let value = js!{
                var link = @{page_link};
                var result = link.match("<https:\\/\\/[\\w\\/\\.]+\\?page=(\\d+)[^>]*>;\\s*rel=\"last\"");
                if(result && result[1]) {
                  var x = 1 * result[1];
                  return x !== x ? 1 : x;
                }else {
                  return 1;
                }
              };
              let last:u32 = value.try_into().unwrap_or(1);
              // trace!("match result is {:?}", self.pagination.last);
              ret.push(Msg::SetPagination(types::Pagination{ current: 0, last, per_page: 0}))
            } else {
              ret.push(Msg::SetPagination(types::Pagination::default()))
            }
            if let Ok(posts) = body {
              // ret.push(Msg::SetPagination(1, 30, 1));
              ret.push(Msg::FetchReady(posts));
            } else {
              ret.push(Msg::Error(format!("Parsing Error: {}", body.unwrap_err(),)));
            }
            ret
          },
        );
        self.state.ft = Some(self.fetch_service.fetch_posts(&self.pagination, callback));
      }
      Msg::FetchReady(result) => {
        self.error = None;
        self.state.posts = result
          .into_iter()
          .map(|post| types::Post {
            labels: post
              .labels
              .clone()
              .into_iter()
              .filter(|label| label.name != "blog")
              .collect(),
            ..post
          })
          .collect();
      }
      Msg::SetPagination(pagination) => {
        macro_rules! check_and_set {
          ($k: ident) => {
            if pagination.$k != 0 {
              self.pagination.$k = pagination.$k;
            }
          };
        };
        check_and_set! {current};
        check_and_set! {last};
        check_and_set! {per_page};
        // correct the last
        if self.pagination.last < self.pagination.current {
          self.pagination.last = self.pagination.current
        }
      }

      Msg::Error(reason) => {
        self.error = Some(reason);
        self.state.posts = Vec::default();
      }
    };
    true
  }

  fn mounted(&mut self) -> ShouldRender {
    self.link.send_self(Msg::FetchPosts);
    false
  }

  fn view(&self) -> Html<Self> {
    html!(
      <div id="main">
        <Loading loading=self.get_fetching() />
        // {self.get_total()}
        <div class="post__wrap">
        // TODO: make post not clone.
        {for self.state.posts.iter().map(move |post| html!{<PostItem is_preview=true post=post.clone() />} )}
        <Pagination
          pagination=self.pagination.clone()
          on_page_change=|x:u32| Msg::SetPagination(types::Pagination{
            current: x,
            per_page: 0,
            last: 0})
        />
        </div>

        <Errors error=self.error.clone() />
      </div>
    )
  }
}

impl Posts {
  fn get_fetching(&self) -> bool {
    if let Some(task) = &self.state.ft {
      task.is_active()
    } else {
      false
    }
  }
}
