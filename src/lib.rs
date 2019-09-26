#![recursion_limit = "512"]

#[macro_use]
extern crate stdweb;

mod app;
mod common;
mod components;
mod utils;
use stdweb::web::{document, IParentNode};
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
  utils::set_panic_hook();
  web_logger::init();
  yew::initialize();
  let app = document().query_selector("body #app").unwrap().unwrap();
  yew::App::<app::App>::new().mount(app);
  yew::run_loop();
  Ok(())
}
