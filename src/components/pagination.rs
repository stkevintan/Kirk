use crate::common::types;
use log::*;
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
  TurnLeft,
  TurnRight,
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
        self.props.on_page_change.emit(target);
        true
      }
      Msg::TurnLeft => {
        trace!("turn left, current is {}", self.props.pagination.current);
        if self.props.pagination.current > 1 {
          self
            .props
            .on_page_change
            .emit(self.props.pagination.current - 1);
        }
        false
      }
      Msg::TurnRight => {
        if self.props.pagination.current < self.props.pagination.last {
          self
            .props
            .on_page_change
            .emit(self.props.pagination.current + 1);
        }
        false
      }
    }
  }

  fn view(&self) -> Html<Self> {
    let is_first = self.props.pagination.current == 1;
    let is_last = self.props.pagination.current == self.props.pagination.last;
    html! {
      <div class="pagination-wrap">
        <ul class="pagination">
        <li
          onclick=|_|Msg::TurnLeft
          class=format!("pagination-item pagination-chevron pagination-chevron-left {}", if is_first {"disabled"} else {""})>
            <i class="iconfont icon-chevron-left"/>
          </li>
        {
          for self.get_index().map(move |current| {
            if current == -1 {
                return html! {
                  <li class="pagination-item pagination-ellipsis pagination-ellipsis-left"><i  class="iconfont icon-dots-horizontal" /></li>
                }
            } else if current == 0  {
               return html! {
                <li class="pagination-item pagination-ellipsis pagination-ellipsis-right"><i  class="iconfont icon-dots-horizontal" /></li>
              }
            }else {
               return  html! {
                <li onclick=|_|Msg::TurnPage(current as u32) class={format!("pagination-item pagination-{}", self.item_class(current as u32))}>{current}</li>
              }
            }
          })
        }
        <li onclick=|_|Msg::TurnRight class=format!("pagination-item pagination-chevron pagination-chevron-right {}", if is_last {"disabled"} else {""})>
            <i class="iconfont icon-chevron-right"/>
        </li>
        </ul>
      </div>
    }
  }
}

impl Pagination {
  fn get_index(&self) -> Box<dyn Iterator<Item = i32>> {
    let current = self.props.pagination.current as i32;
    let last = self.props.pagination.last as i32;
    // if pages is lower than 10, ouput it directly
    if last < 10 {
      return Box::new(1..=last);
    }
    let head = vec![1, -1];
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
