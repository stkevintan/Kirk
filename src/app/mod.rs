use super::common::*;
#[warn(unused_must_use)]
use failure::Error;
use log::*;
use serde_derive::Deserialize;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

mod search;
use search::Search;

pub struct App {
  link: ComponentLink<App>,
  fetch_service: FetchService,
  state: State,
  ft: Option<FetchTask>,
}

static POSTS_URL: &str = "https://yapi.bytedance.net/mock/844/xztech/blog/v1/posts/";

#[derive(Deserialize, Debug)]
pub struct Category {
  id: i32,
  name: String,
}

#[derive(Deserialize, Debug)]
pub struct Post {
  post_id: i32,
  create_at: String,
  last_modified_at: String,
  title: String,
  author_id: i32,
  author_name: String,
  summary: String,
  categories: Vec<Category>,
  tags: Vec<String>,
  is_draft: bool,
  covers: Vec<String>,
}

#[derive(Deserialize)]
pub struct State {
  post_result: Option<PageResult<Post>>,
}

pub enum Msg {
  FetchPosts,
  FetchReady(Option<PageResult<Post>>),
  Nope,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    let state = State { post_result: None };
    App {
      ft: None,
      link,
      fetch_service: FetchService::new(),
      state,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::FetchPosts => {
        let callback = self.link.send_back(
          move |response: Response<Json<Result<API<PageResult<Post>>, Error>>>| {
            let (header, Json(body)) = response.into_parts();
            if let Err(err) = &body {
              trace!("error: {:?}", err);
            } else {
              trace!("data: {:?}", body);
            }
            if !header.status.is_success() {
              return Msg::Nope;
            }
            if let Ok(API { data, code, .. }) = body {
              if code != 0 {
                return Msg::Nope;
              }
              Msg::FetchReady(data)
            } else {
              Msg::Nope
            }
          },
        );
        let request = Request::get(POSTS_URL).body(Nothing).unwrap();
        let task = self.fetch_service.fetch(request, callback);
        self.ft = Some(task);
      }
      Msg::FetchReady(result) => {
        trace!("self.state.post_result {:?}", result);
        self.state.post_result = result;
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

impl Renderable<App> for App {
  fn view(&self) -> Html<Self> {
    info!("rendered!");
    html! {
    <section id="layout">
      { self.header() }
      <div id="body">
        { self.sidebar() }
        { self.main() }
      </div>
    </section>
    }
  }
}

impl App {
  fn get_total(&self) -> i32 {
    trace!("post_result: {:?}", self.state.post_result);
    self
      .state
      .post_result
      .as_ref()
      .map(|x| x.total)
      .unwrap_or(0)
  }

  fn header(&self) -> Html<App> {
    html! {
      <section id="header">
        <div class="header-logo">{ "Kirk" } </div>
        <div class="flex-grow" />
      </section>
    }
  }

  fn main(&self) -> Html<App> {
    html!(
      <div id="main">
        {self.get_total()}
      </div>
    )
  }

  fn sidebar(&self) -> Html<App> {
    html!(
      <div id="sidebar">
         <Search placeholder="Search keywords..." />
         <ul>
          <li>{ "Home" }</li>
          <li>{ "Archive" }</li>
          <li>{ "About" }</li>
        </ul>
      </div>
    )
  }
}
