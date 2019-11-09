use yew::prelude::*;

pub struct Search {
  value: String,
  placeholder: String,
  onclick: Option<Callback<String>>,
}

pub enum Msg {
  GotInput(String),
  OnSearch,
}

#[derive(PartialEq, Properties)]
pub struct Props {
  pub initial: String,
  pub placeholder: String,
  pub onclick: Option<Callback<String>>,
}

impl Default for Props {
  fn default() -> Self {
    Props {
      initial: "".to_string(),
      placeholder: "".to_owned(),
      onclick: None,
    }
  }
}

impl Component for Search {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {
      value: props.initial,
      onclick: props.onclick,
      placeholder: props.placeholder,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::GotInput(new_value) => self.value = new_value,
      Msg::OnSearch => {
        if let Some(handler) = &self.onclick {
          handler.emit(self.value.clone());
        }
      }
    };
    true
  }
}

impl Renderable<Search> for Search {
  fn view(&self) -> Html<Self> {
    html! {
      <div class="input__search">
        <span class="iconfont icon-magnify" onclick=|_| Msg::OnSearch />
        <input value=&self.value oninput=|e| Msg::GotInput(e.value) />
        { self.view_placeholder() }
      </div>
    }
  }
}

impl Search {
  fn view_placeholder(&self) -> Html<Self> {
    if self.placeholder.len() > 0 && self.value.len() == 0 {
      html! {
        <span class="input__search--placeholder">{&self.placeholder}</span>
      }
    } else {
      html! {}
    }
  }
}
