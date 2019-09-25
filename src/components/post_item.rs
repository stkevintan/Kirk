use crate::common::types;
use yew::prelude::*;

pub struct PostItem {
  post: types::Post,
  style: String,
  class: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
  #[props(required)]
  pub post: types::Post,
  pub class: String,
  pub style: String,
}

impl Component for PostItem {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {
      post: props.post,
      class: props.class,
      style: props.style,
    }
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    false
  }

  // fn change(&mut self, props: Self::Properties) -> ShouldRender {
  //   self.post = props.post;
  //   self.class = props.class;
  //   self.style = props.style;
  //   true
  // }
}

impl PostItem {
  fn header_view(&self) -> Html<Self> {
    html! {
      <div class="post-item__header">
      <div class="post-item__header--title">{&self.post.title}</div>
      <ul class="post-item__header--meta">
        <li class="post-item__meta--author">
          <span data-src={&self.post.user.avatar_url} class="post-item__meta--avator"  />
          <span class="post-item__meta--name">{"Author:" }{&self.post.user.login}</span>
        </li>
        <li class="post-item__meta--created-at">{"Created At: "}{&self.post.created_at}</li>
        <li class="post-item__meta--updated-at">{"Updated At: "}{&self.post.updated_at}</li>
      </ul>
      <ul class="post-item__header--tags">
        {for self.post.labels.iter().map(|label| html! {
          <li key={&label.id} style=format!("style: {}", label.color)>{&label.name}</li>
        })}
      </ul>
      </div>
    }
  }
  fn body_view(&self) -> Html<Self> {
    html! {
      <section class="post-item__body">
        {&self.post.body}
      </section>
    }
  }
}
impl Renderable<PostItem> for PostItem {
  fn view(&self) -> Html<Self> {
    html! {
      <div
         class=format!("post-item {}", self.class)
         style=self.style
      >
        {self.header_view()}
        {self.body_view()}
      </div>
    }
  }
}
