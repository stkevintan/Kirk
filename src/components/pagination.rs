use crate::common::types;
use yew::prelude::*;
use yew::Callback;
#[derive(PartialEq, Properties)]
pub struct Props {
  pub pagination: types::Pagination,
  #[props(required)]
  pub on_page_change: Callback<u32>,
}

pub struct Pagination {
  props: Props,
}

pub enum Msg {
  TurnPage(u32),
}

impl Component for Pagination {
  type Message = Msg;
  type Properties = Props;
  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self { props }
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.props = props;
    true
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::TurnPage(target) => {
        if self.props.pagination.current == target {
          return false;
        }
        self.props.on_page_change.emit(target)
      }
    }
    true
  }

  fn view(&self) -> Html<Self> {
    html! {
      <div class="pagination-wrap">
        <ul class="pagination">
        {
          for self.get_index().map(move |current| {
            html! {
              <li onclick=|_|Msg::TurnPage(current) class="pagination-item">{current}</li>
            }
          })
        }
        </ul>
      </div>
    }
  }
}

impl Pagination {
  fn get_index(&self) -> Box<dyn Iterator<Item = u32>> {
    let current = self.props.pagination.current;
    let last = self.props.pagination.last;
    // if pages is lower than 10, ouput it directly
    if last < 10 {
      return Box::new(1..=last);
    }
    let head = vec![1, 0];
    let tail = vec![0, last];
    if current < 4 {
      return Box::new((1..5).chain(tail.clone()));
    } else if current == 4 {
      return Box::new((1..=5).chain(tail.clone()));
    }

    if current > last - 3 {
      return Box::new(head.clone().into_iter().chain((last - 3)..=last));
    } else if current == last - 3 {
      return Box::new(head.clone().into_iter().chain((last - 4)..=last));
    }

    return Box::new(
      head
        .clone()
        .into_iter()
        .chain((current - 1)..=(current + 1))
        .chain(tail.clone()),
    );
  }
  fn item_class(&self, page_index: u32) -> String {
    if page_index == self.props.pagination.current {
      "active".into()
    } else {
      "default".into()
    }
  }
}
