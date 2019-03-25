#![feature(
  proc_macro_hygiene,
  slice_patterns,
  custom_attribute,
  extern_crate_item_prelude,
  bind_by_move_pattern_guards
)]

use wasm_bindgen::prelude::*;
use web_sys::{
  Document,
  Element,
};

mod api;
mod app;
mod detail_view;
mod home;
mod input;
mod types;
mod util;

#[wasm_bindgen]
pub fn start(root_element: web_sys::Element) {
  let app_state = types::AppState::new();
  let app = app::render(app_state);

  smithy::mount(Box::new(app), root_element);
}
