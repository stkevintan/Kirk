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
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct About {
  link: ComponentLink<About>,
  fetch_service: CustomFetchService,
  error: Option<String>,
  post: Option<types::Post>,
  ft: Option<FetchTask>,
}

pub enum Msg {
  FetchAbout,
  FetchReady(types::Post),
  Error(String),
}

impl Component for About {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      link,
      fetch_service: CustomFetchService::default(),
      error: None,
      post: None,
      ft: None,
    }
  }
  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::FetchAbout => {
        let callback = self.link.send_back(
          move |response: Response<Json<Result<Vec<types::Post>, Error>>>| {
            let (headers, Json(body)) = response.into_parts();
            if !headers.status.is_success() {
              return Msg::Error(format!(
                "Response with error header: {}",
                headers.status.as_str()
              ));
            }
            if let Ok(posts) = body {
              if posts.len() == 0 {
                return Msg::Error(format!("About me is not found"));
              }
              return Msg::FetchReady(posts[0].clone());
            }
            return Msg::Error(format!("Parsing error: {}", body.unwrap_err()));
          },
        );
        self.ft = Some(self.fetch_service.fetch_about(callback));
      }
      Msg::FetchReady(post) => {
        self.error = None;
        self.post = Some(types::Post {
          labels: post
            .labels
            .clone()
            .into_iter()
            .filter(|label| label.name != "about")
            .collect(),
          ..post
        })
      }
      Msg::Error(err) => self.error = Some(err),
    };
    true
  }

  fn mounted(&mut self) -> ShouldRender {
    self.link.send_self(Msg::FetchAbout);
    false
  }

  fn view(&self) -> Html<Self> {
    html!(
      <div id="main">
        <Loading loading=self.get_fetching() />
        <Errors error=self.error.clone() />
        {
          if let Some(post) = &self.post.as_ref() {
            html!{
              <div class="post__wrap post-single">
                <PostItem is_preview=false is_about=true post=post.clone() />
                <Comments id=post.number count=post.comments />
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

impl About {
  fn get_fetching(&self) -> bool {
    if let Some(task) = &self.ft {
      task.is_active()
    } else {
      false
    }
  }
}
