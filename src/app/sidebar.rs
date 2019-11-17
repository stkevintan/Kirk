use crate::components::Search;
use yew::prelude::*;
use yew_router::{agent::RouteRequest, prelude::*};
pub enum Msg {
  Nope,
  Navigate(&'static str),
}

pub struct Sidebar {
  router: Box<dyn Bridge<RouteAgent>>,
}

impl Component for Sidebar {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
    let callback = link.send_back(|_| Msg::Nope);
    let router = RouteAgent::bridge(callback);
    Self { router }
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
       <Search placeholder="Search keywords..." />
       <ul>
        <li class="active" onclick=|_| Msg::Navigate("/") ><i class="iconfont icon-home" />
        { "Home" }
        </li>
        <li><i class="iconfont icon-archive" />{ "Archive" }</li>
        <li><i class="iconfont icon-account" />{ "About Me" }</li>
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
