use crate::common::is_light_color;
use crate::common::types;
use pulldown_cmark::{html, Options, Parser};
use stdweb::js;
use stdweb::unstable::TryFrom;
use stdweb::web::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::*;
pub struct PostItem {
  props: Props,
}

#[derive(Properties, PartialEq)]
pub struct Props {
  #[props(required)]
  pub post: types::Post,
  pub class: String,
  pub style: String,
  pub is_preview: bool,
}

impl Component for PostItem {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self { props }
  }

  fn mounted(&mut self) -> ShouldRender {
    js! {@(no_return)
      // TODO: replace it with rs highlight tool
      if (window.Prism) {
        window.Prism.highlightAll(false);
      }
    }
    false
  }
  fn update(&mut self, _: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props = props;
    true
  }

  fn view(&self) -> Html<Self> {
    html! {
      <article
         class=format!("post-item {}", self.props.class)
         style=self.props.style
      >
        {self.header_view()}
        {self.body_view()}
      </article>
    }
  }
}

impl PostItem {
  fn parse_markdown(&self) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(self.props.post.body.as_str(), options);

    let mut html_str = String::new();
    html::push_html(&mut html_str, parser);
    html_str
  }
  fn markdown_view(&self) -> Html<Self> {
    let render = js! {
      var div = document.createElement("div");
      var script = document.createElement("script");
      div.className = "markdown__body";
      div.innerHTML = @{self.parse_markdown()};
      script.innerHTML = "if(window.Prism) { window.Prism.highlightAll(true); }";
      div.appendChild(script);

      return div;
    };
    if let Ok(node) = Node::try_from(render) {
      let vnode = VNode::VRef(node);
      vnode
    } else {
      html! {
        <div class="error">{"error"}</div>
      }
    }
  }
  fn header_view(&self) -> Html<Self> {
    html! {
      <div class="post-item__header">
      <div class="post-item__header__title">
      {
        if self.props.is_preview {
          html!(<RouterLink: text=&self.props.post.title, link=format!("/posts/{}", self.props.post.number), />)
        }else {
          html!{&self.props.post.title}
        }
      }
      </div>
      <ul class="post-item__header__meta">
        <li class="post-item__header__author">
          <span style={format!("background-image: url({});", self.props.post.user.avatar_url)} class="post-item__header__avator"  />
          <span class="post-item__header__name"><a target="_blank" href={format!("{}",self.props.post.user.html_url)}>{&self.props.post.user.login}</a></span>
        </li>
        <li class="post-item__header__updated-at">
              <label><i class="iconfont icon-ic_query_builder_px" /></label>
              <span>{&self.props.post.updated_at}</span>
         </li>
      </ul>
      {self.tags_view()}
      </div>
    }
  }
  fn tags_view(&self) -> Html<Self> {
    if self.props.post.labels.len() == 0 {
      html! {}
    } else {
      html! {
        <ul class="post-item__header__tags">
          {for self.props.post.labels.iter().map(|label| html! {
            <li key={&label.id}>
              <span class="tag" style=format!(
                "background-color: #{}; color: {}",
                label.color,
                if is_light_color(&label.color) { "#333" } else {"#fff"}
              )>{&label.name}</span>
            </li>
          })}
        </ul>
      }
    }
  }
  fn body_view(&self) -> Html<Self> {
    html! {
      <section class=format!("post-item__body post-item__body--{} {}", self.props.post.id, if self.props.is_preview { "preview" } else {""})>
        {self.markdown_view()}
      </section>
    }
  }
}
