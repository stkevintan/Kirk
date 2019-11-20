use crate::components::Sidebar;
use log::*;
use yew::prelude::*;
use yew_router::{agent::RouteRequest, prelude::*};
mod about;
mod post;
mod posts;
use about::About;
use post::Post;
use posts::Posts;

pub struct App {
  location: String,
  router: Box<dyn Bridge<RouteAgent>>,
}

#[derive(Switch, Debug, Clone)]
pub enum PostRoute {
  #[to = "/{id}"]
  Post(u32),
  #[to = "?page={current}"]
  WithPage(u32),
  #[to = "/"]
  Default,
}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
  #[to = "/!"]
  Index,
  #[to = "/posts{*:params}"]
  Posts(PostRoute),
  #[to = "/about"]
  About,
  #[to = "/404"]
  PageNotFound(Option<String>),
}

pub enum Msg {
  GoHome,
  RouteChange(Route),
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
    let callback = link.send_back(|e| Msg::RouteChange(e));
    let router = RouteAgent::bridge(callback);
    Self {
      router,
      location: "/".into(),
    }
  }

  fn mounted(&mut self) -> ShouldRender {
    self.router.send(RouteRequest::GetCurrentRoute);
    false
  }
  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::GoHome => {
        let route = Route::from("/");
        self.router.send(RouteRequest::ChangeRoute(route));
        true
      }
      Msg::RouteChange(route) => {
        self.location = route.route;
        true
      }
    }
  }
  fn view(&self) -> Html<Self> {
    html! {
    <section id="layout">
      { self.header() }
      <div id="body" class="clearfix">
        <Sidebar location=self.location.clone() />
        <Router<AppRoute, ()>
        render = Router::render(|switch: AppRoute| {
          match switch {
            // AppRoute::Post(id) => html!{<Post id=id />},
            AppRoute::Index => html!{<Posts current=1 />},
            AppRoute::Posts(router) => {
              match router {
                PostRoute::Post(id) => html!{<Post id=id />},
                PostRoute::WithPage(current) => html!{<Posts current=current />},
                PostRoute::Default => html!{<Posts current=1 />}
              }
            },
            AppRoute::About => html!{<About />},
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
        <div onclick=|_|Msg::GoHome class="header__logo">
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
}
