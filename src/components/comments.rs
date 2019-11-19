use crate::common::*;
use crate::components::Errors;
use crate::components::Loading;
use failure::Error;
use log::*;
use pulldown_cmark::{html, Options, Parser};
use stdweb::js;
use stdweb::unstable::TryFrom;
use stdweb::web::Node;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchTask, Response};
use yew::services::Task;
use yew::virtual_dom::VNode;

pub struct Comments {
  props: Props,
  comments: Vec<types::Comment>,
  fetch_service: CustomFetchService,
  fetch_token: Option<FetchTask>,
  link: ComponentLink<Comments>,
  error: Option<String>,
  is_open: bool,
  is_fetched: bool,
}

#[derive(Properties)]
pub struct Props {
  #[props(required)]
  pub id: u32,
  pub count: u32,
  // pub link: String,
}

pub enum Msg {
  Toggle,
  StartFetch,
  FetchReady(Vec<types::Comment>),
  Error(String),
}

impl Component for Comments {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      link,
      props,
      comments: Vec::default(),
      error: None,
      fetch_service: CustomFetchService::default(),
      fetch_token: None,
      is_fetched: false,
      is_open: false,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Toggle => {
        if self.props.count == 0 {
          return false;
        }
        self.is_open = !self.is_open;
        if self.is_open && !self.is_fetched {
          self.link.send_self(Msg::StartFetch);
          false
        } else {
          true
        }
      }
      Msg::StartFetch => {
        let callback = self.link.send_back(
          move |response: Response<Json<Result<Vec<types::Comment>, Error>>>| {
            let (header, Json(body)) = response.into_parts();
            if let Err(err) = &body {
              trace!("error: {:?}", err);
            }
            if !header.status.is_success() {
              return Msg::Error(format!(
                "Request failed with status: {}",
                header.status.as_str()
              ));
              // return ret;
            }
            if let Ok(comments) = body {
              return Msg::FetchReady(comments);
            } else {
              return Msg::Error(format!("parsing Error: {}", body.unwrap_err()));
            }
          },
        );
        self.fetch_token = Some(self.fetch_service.fetch_comments(self.props.id, callback));
        true
      }
      Msg::FetchReady(comments) => {
        self.error = None;
        self.comments = comments;
        self.is_fetched = true;
        true
      }
      Msg::Error(reason) => {
        self.error = Some(reason);
        self.comments = Vec::default();
        self.is_fetched = false;
        true
      }
    }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props = props;
    true
  }

  fn mounted(&mut self) -> ShouldRender {
    // self.link.send_self(Msg::StartFetch);
    false
  }

  fn view(&self) -> Html<Self> {
    html! {
      <div class=format!("comments {}", self.get_status())>
        <div class="comment-header">
          <h2 class="comment-header-title" onclick=|_|Msg::Toggle>{"Comments"}</h2>
          <span class="comment-header-count">{format!("({})", self.props.count)}</span>
          <span class="flex-grow" />
          <a href="javascript:void(0)" class="comment-toggle" onclick=|_|Msg::Toggle>
            <i class="iconfont icon-chevron-double-right"/>
          </a>
        </div>
        <div class="comment-list">
          <Loading loading=self.get_fetching() />
          <Errors error=self.error.clone() />
          {for self.comments.iter().map(move |comment| html!{
            <div class="comment">
              {self.view_meta(&comment)}
              {self.view_main(&comment)}
            </div>
          })}
        </div>
      </div>
    }
  }
}

impl Comments {
  fn get_fetching(&self) -> bool {
    if let Some(task) = &self.fetch_token {
      task.is_active()
    } else {
      false
    }
  }

  fn get_status(&self) -> String {
    if self.props.count == 0 {
      "disabled".into()
    } else if self.is_open {
      "on".into()
    } else {
      "off".into()
    }
  }
  fn view_meta(&self, comment: &types::Comment) -> Html<Self> {
    html! {
      <div class="comment-meta">
        <span
        style={format!("background-image: url({})", comment.user.avatar_url)}
        class="comment-meta-avatar"
        />
        <span class="comment-meta-name">
          <a target="_blank" href=comment.user.html_url.as_str()>
            {&comment.user.login}
          </a>
        </span>
        <span class="comment-meta-associate">
          {&comment.author_association}
        </span>
        <a href=comment.html_url.as_str() target="_blank">
        <i class="iconfont icon-link-variant" />
        </a>
        <span class="flex-grow" />
        <span class="comment-meta-date">
          <label><i class="iconfont icon-ic_query_builder_px"/></label>
          <span>{&comment.updated_at}</span>
        </span>
      </div>
    }
  }

  fn view_main(&self, comment: &types::Comment) -> Html<Self> {
    html! {
      <section class=format!("comment-body comment-body-{}", self.props.id)>
        {self.markdown_view(&comment.body)}
      </section>
    }
  }

  fn parse_markdown(&self, body: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(body, options);

    let mut html_str = String::new();
    html::push_html(&mut html_str, parser);
    html_str
  }
  fn markdown_view(&self, body: &str) -> Html<Self> {
    let render = js! {
      var div = document.createElement("div");
      var script = document.createElement("script");
      div.className = "markdown__body";
      div.innerHTML = @{self.parse_markdown(body)};
      script.innerHTML = "if(window.Prism) { window.Prism.highlightAll(true); }";
      div.appendChild(script);
      return div;
    };
    if let Ok(node) = Node::try_from(render) {
      VNode::VRef(node)
    } else {
      html! {
        <div class="error">{"error"}</div>
      }
    }
  }
}
