use log::*;
use yew::{html, Component, ComponentLink, Href, Html, Renderable, ShouldRender};
use serde_derive::{Deserialize, Serialize};

pub struct App {
    state: State
}


#[derive(Serialize, Deserialize)]
pub struct State {
    count: i32
}

impl State {
    fn get_count(&self) -> i32 {
        self.count
    }
}

pub enum Msg {
    Add(i32),
    Subtract(i32),
    Nope
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let state = State {
            count: 0
        };
        App {state}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add(x) => self.state.count += x,
            Msg::Subtract(x) => self.state.count -= x,
            Msg::Nope => {}
        };
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        info!("rendered!");
        html! {
            <div class="layout">
                <section class="header">
                    <div class="header-logo">{ "Kirk" }</div>
                    <ul class="header-menu">
                        <li class="active">{"Posts"}</li>
                        <li>{"Archive"}</li>
                        <li>{"About"}</li>
                    </ul>
                </section>
                <section class="body">
                {"the counter: "} {self.state.get_count()}
                </section>
            </div>
        }
    }
}

