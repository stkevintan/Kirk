use yew::prelude::*;

pub struct Loading {
  loading: bool,
  class: String,
  style: String,
}

#[derive(Default, Properties, PartialEq)]
pub struct Props {
  pub loading: bool,
  pub class: String,
  pub style: String,
}

impl Component for Loading {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Loading {
      loading: props.loading,
      class: props.class,
      style: props.style,
    }
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.loading = props.loading;
    self.class = props.class;
    self.style = props.style;
    true
  }
}

impl Renderable<Loading> for Loading {
  fn view(&self) -> Html<Self> {
    html! {
      <div
         class=format!("loading loading-{} {}", if self.loading { "active" } else { "disabled" }, self.class)
         style=self.style
      >
      <svg class="spinner" width="65px" height="65px" viewBox="0 0 66 66" xmlns="http://www.w3.org/2000/svg">
         <circle class="path" fill="none" stroke-width="6" stroke-linecap="round" cx="33" cy="33" r="30"></circle>
      </svg>
      </div>
    }
  }
}
