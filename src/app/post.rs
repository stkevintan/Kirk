use crate::common::*;
use crate::components::Loading;
use crate::components::PostItem;

#[warn(unused_must_use)]
use failure::Error;
use log::*;
use yew::format::Json;
use yew::services::fetch::{FetchTask, Response};
use yew::services::Task;
use yew::{html, prelude::*, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Post {
  link: ComponentLink<Post>,
  fetch_service: CustomFetchService,
  state: State,
  id: u32,
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
  Error,
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
            let Json(body) = response.into_body();
            if let Err(err) = &body {
              trace!("error: {:?}", err);
            }
            if let Ok(post) = body {
              return Msg::FetchReady(post);
            }
            return Msg::Error;
          },
        );
        self.state.ft = Some(self.fetch_service.fetch_post(self.id, callback));
      }
      Msg::FetchReady(post) => {
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
      Msg::Error => {}
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
        {
          if let Some(post) = self.state.post.clone() {
            html!{
              <div class="post__wrap">
              <PostItem is_preview=false post=post />
              </div>
            }
          } else {
            html!{"parse error!"}
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
