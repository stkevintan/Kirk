use super::components::Search;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{prelude::*, Switch};

mod posts;
use posts::Posts;

mod post;
use post::Post;
pub struct App {}

pub enum Msg {
  Nope,
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
  #[to = "/posts/{id}"]
  Post(u32),
  #[to = "/!"]
  Index,
  #[to = "/404"]
  PageNotFound(Option<String>),
}

impl Component for App {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
    App {}
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Nope => {}
    };
    true
  }
  fn view(&self) -> Html<Self> {
    html! {
    <section id="layout">
      { self.header() }
      <div id="body" class="clearfix">
        { self.sidebar() }
        <Router<AppRoute, ()>
        render = Router::render(|switch: AppRoute| {
          match switch {
            AppRoute::Post(id) => html!{<Post id=id />},
            AppRoute::Index => html!{<Posts />},
            AppRoute::PageNotFound(x) => html!{format!("404 url: {:?}", x)}
          }
        })
        redirect = Router::redirect(|route: Route| {
          AppRoute::PageNotFound(Some(route.route))
        })
        />
      </div>
    </section>
    }
  }
}

impl App {
  fn header(&self) -> Html<App> {
    html! {
      <section id="header">
        <div class="header__logo">
        <span class="img-spaceship" />
        <span>{ "Kirk" }</span>
        </div>
        <div class="flex-grow" />
        <ul class="header__menus">
          <li><i class="iconfont icon-github-circle" /></li>
          <li><i class="iconfont icon-rss" /></li>
        </ul>
      </section>
    }
  }

  fn sidebar(&self) -> Html<App> {
    html!(
      <div id="sidebar">
         <Search placeholder="Search keywords..." />
         <ul>
          <li class="active"><i class="iconfont icon-home" />
          // { "Home" }
          <RouterLink: text="Home" link="/" />
          </li>
          <li><i class="iconfont icon-archive" />{ "Archive" }</li>
          <li><i class="iconfont icon-account" />{ "About Me" }</li>
        </ul>
      </div>
    )
  }
}
