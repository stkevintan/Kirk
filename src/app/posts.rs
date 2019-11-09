use crate::common::*;
use crate::components::Loading;
use crate::components::PostItem;

#[warn(unused_must_use)]
use failure::Error;
use log::*;
use yew::format::Json;
use yew::services::fetch::{FetchTask, Response};
use yew::services::Task;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Posts {
  link: ComponentLink<Posts>,
  fetch_service: CustomFetchService,
  state: State,
}

#[derive(Default)]
pub struct State {
  posts: Vec<types::Post>,
  ft: Option<FetchTask>,
  per_page: u32,
  page: u32,
  last_page: u32,
}

impl std::fmt::Debug for State {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    writeln!(f, "Posts: {:?}", self.posts)?;
    writeln!(
      f,
      "per_page {} page {}, last_page {}",
      self.per_page, self.page, self.last_page
    )
  }
}

pub enum Msg {
  FetchPosts,
  FetchReady(Vec<types::Post>),
  SetPagination(u32, u32, u32),
  Nope,
}

impl Component for Posts {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let state = State::default();
    Posts {
      link,
      fetch_service: CustomFetchService::default(),
      state,
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
              ret.push(Msg::Nope);
              return ret;
            }

            let headers = &header.headers;
            if !headers.contains_key("Link") {
              //
            } else {
              let link = &headers["Link"];
              trace!("{:?}", link);
            }

            if let Ok(posts) = body {
              ret.push(Msg::SetPagination(1, 30, 1));
              ret.push(Msg::FetchReady(posts));
            } else {
              ret.push(Msg::Nope);
            }
            ret
          },
        );
        self.state.ft = Some(self.fetch_service.fetch_posts(callback));
      }
      Msg::FetchReady(result) => {
        trace!("fetchReady {:?}", self.state);
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
      Msg::SetPagination(page, per_page, last_page) => {
        trace!("setPagination {:?}", self.state);
        self.state.page = page;
        self.state.per_page = per_page;
        self.state.last_page = last_page;
      }
      Msg::Nope => {}
    };
    true
  }

  fn mounted(&mut self) -> ShouldRender {
    self.link.send_self(Msg::FetchPosts);
    false
  }
}

impl<'a> Renderable<Posts> for Posts {
  fn view(&self) -> Html<Self> {
    html!(
      <div id="main">
        <Loading loading=self.get_fetching() />
        // {self.get_total()}
        <div class="post-list">
        {for self.state.posts.iter().map(move |post| html!{<PostItem post=post.clone() />} )}
        </div>
      </div>
    )
  }
}

impl Posts {
  fn get_total(&self) -> usize {
    trace!("post_result: {:?}", self.state.posts);
    self.state.posts.len()
  }

  fn get_fetching(&self) -> bool {
    if let Some(task) = &self.state.ft {
      task.is_active()
    } else {
      false
    }
  }
}
