#![recursion_limit = "512"]
mod app;
mod common;
mod utils;

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
  // log::info!("start");
  yew::start_app::<app::App>();
  // run..send_message(app::Msg::FetchPosts);
  // app::App::fetch();
  Ok(())
}
