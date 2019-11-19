use yew::prelude::*;
use yew_router::prelude::*;

mod posts;
use posts::Posts;

mod post;
use post::Post;

mod sidebar;
use sidebar::Sidebar;
pub struct App {}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
  #[to = "/posts/{id}"]
  Post(u32),
  #[to = "/!"]
  Index,
  #[to = "/posts?page={current}"]
  Posts(u32),
  #[to = "/404"]
  PageNotFound(Option<String>),
}

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_: Self::Properties, mut _link: ComponentLink<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }
  fn view(&self) -> Html<Self> {
    html! {
    <section id="layout">
      { self.header() }
      <div id="body" class="clearfix">
        <Sidebar />
        <Router<AppRoute, ()>
        render = Router::render(|switch: AppRoute| {
          match switch {
            AppRoute::Post(id) => html!{<Post id=id />},
            AppRoute::Index => html!{<Posts current=1 />},
            AppRoute::Posts(current) => html!{<Posts current=current />},
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
}
