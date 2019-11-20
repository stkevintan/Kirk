// use crate::components::Search;
use yew::prelude::*;
use yew_router::{agent::RouteRequest, prelude::*};
pub enum Msg {
  Nope,
  Navigate(&'static str),
}

#[derive(Properties)]
pub struct Props {
  pub location: String,
}

pub struct Sidebar {
  props: Props,
  router: Box<dyn Bridge<RouteAgent>>,
}

impl Component for Sidebar {
  type Message = Msg;
  type Properties = Props;
  fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
    let callback = link.send_back(|_| Msg::Nope);
    let router = RouteAgent::bridge(callback);
    Self { router, props }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props = props;
    true
  }
  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Nope => false,
      Msg::Navigate(url) => {
        let route = Route::from(url);
        self.router.send(RouteRequest::ChangeRoute(route));
        true
      }
    }
  }

  fn view(&self) -> Html<Self> {
    html! {
       <div id="sidebar">
      //  <Search placeholder="Search keywords..." />
       <ul>
        <li>
        <a class=if self.is_post_page() {"active"} else {""} onclick=|_| Msg::Navigate("/") href="javascript:void(0)">
          <i class="iconfont icon-home" />
          { "Home" }
        </a>
        </li>
        <li>
        <a href="https://github.com/stkevintan/kirk-blog/issues" target="_blank">
          <i class="iconfont icon-archive" />
          { "Archive" }
          <span class="flex-grow" />
          <i class="link-color iconfont icon-link-variant"/>
        </a>
        </li>
        <li>
        <a class=if self.is_about_page() {"active"} else {""}  onclick=|_| Msg::Navigate("/about") href="javascript:void(0)">
          <i class="iconfont icon-account" />
          { "About Me" }
        </a>
        </li>
      </ul>
      <div class="footer">
        {"Powered by "}
        <a href="https://github.com/yewstack/yew" target="_blank">{"Yew"}</a>
        {" and Rust WebAssembly"}
      </div>
    </div>
     }
  }
}

impl Sidebar {
  fn is_post_page(&self) -> bool {
    self.props.location == "/" || self.props.location.starts_with("/posts")
  }
  fn is_about_page(&self) -> bool {
    self.props.location.starts_with("/about")
  }
}
