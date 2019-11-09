use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use super::components::Search;

mod posts;
use posts::Posts;

pub struct App {}

pub enum Msg {
  Nope,
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
}

impl Renderable<App> for App {
  fn view(&self) -> Html<Self> {
    html! {
    <section id="layout">
      { self.header() }
      <div id="body" class="clearfix">
        { self.sidebar() }
        <Posts />
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
          <li class="active"><i class="iconfont icon-home" />{ "Home" }</li>
          <li><i class="iconfont icon-archive" />{ "Archive" }</li>
          <li><i class="iconfont icon-account" />{ "About Me" }</li>
        </ul>
      </div>
    )
  }
}
