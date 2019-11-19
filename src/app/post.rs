use crate::common::*;
use crate::components::Comments;
use crate::components::Errors;
use crate::components::Loading;
use crate::components::PostItem;

#[warn(unused_must_use)]
use failure::Error;
use log::*;
use yew::format::Json;
use yew::services::fetch::{FetchTask, Response};
use yew::services::Task;
use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub struct Post {
  link: ComponentLink<Post>,
  fetch_service: CustomFetchService,
  state: State,
  id: u32,
  error: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
  #[props(required)]
  pub id: u32,
}

#[derive(Default)]
pub struct State {
  post: Option<types::Post>,
  ft: Option<FetchTask>,
}

pub enum Msg {
  FetchPost,
  FetchReady(types::Post),
  Error(String),
}

impl Component for Post {
  type Message = Msg;
  type Properties = Props;
  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let state: State = State::default();
    Self {
      link,
      fetch_service: CustomFetchService::default(),
      state,
      error: None,
      id: props.id,
    }
  }
  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if self.id != props.id {
      self.id = props.id;
      self.link.send_self(Msg::FetchPost);
    }
    false
  }
  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::FetchPost => {
        let callback = self.link.send_back(
          move |response: Response<Json<Result<types::Post, Error>>>| {
            let (headers, Json(body)) = response.into_parts();
            if !headers.status.is_success() {
              return Msg::Error(format!(
                "Response with error header: {}",
                headers.status.as_str()
              ));
            }
            if let Ok(post) = body {
              return Msg::FetchReady(post);
            }
            return Msg::Error(format!("Parsing error: {}", body.unwrap_err()));
          },
        );
        self.state.ft = Some(self.fetch_service.fetch_post(self.id, callback));
      }
      Msg::FetchReady(post) => {
        self.error = None;
        self.state.post = Some(types::Post {
          labels: post
            .labels
            .clone()
            .into_iter()
            .filter(|label| label.name != "blog")
            .collect(),
          ..post
        })
      }
      Msg::Error(err) => self.error = Some(err),
    };
    true
  }

  fn mounted(&mut self) -> ShouldRender {
    self.link.send_self(Msg::FetchPost);
    false
  }

  fn view(&self) -> Html<Self> {
    html!(
      <div id="main">
        <Loading loading=self.get_fetching() />
        <Errors error=self.error.clone() />
        {
          if let Some(post) = &self.state.post.as_ref() {
            html!{
              <div class="post__wrap">
                <PostItem is_preview=false post=post.clone() />
                <Comments id=self.id count=post.comments />
              </div>
            }
          } else {
            html!{}
          }
        }
      </div>
    )
  }
}

impl Post {
  fn get_fetching(&self) -> bool {
    if let Some(task) = &self.state.ft {
      task.is_active()
    } else {
      false
    }
  }
}
