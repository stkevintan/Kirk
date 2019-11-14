use yew::prelude::*;

pub struct Errors {
  error: Option<String>,
  class: String,
  style: String,
}

#[derive(Default, Properties, PartialEq)]
pub struct Props {
  pub error: Option<String>,
  pub class: String,
  pub style: String,
}

impl Component for Errors {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {
      error: props.error,
      class: props.class,
      style: props.style,
    }
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.error = props.error;
    self.class = props.class;
    self.style = props.style;
    true
  }

  fn view(&self) -> Html<Self> {
    html! {
      <div
         class=format!("error error-{} {}", if self.has_error() { "show" } else { "hide" }, self.class)
         style=self.style
      >
      <div class="error-inner">
        <h1>{"(╯°□°）╯︵ ┻━┻ "}</h1>
        <h1>{"An Error has occurred"}</h1>
        {
          if let Some(err) = &self.error.as_ref() {
            html!(<p>{err}</p>)
          }else {
            html!()
          }
        }
      </div>
     </div>
    }
  }
}

impl Errors {
  fn has_error(&self) -> bool {
    self.error.is_some()
  }
}
