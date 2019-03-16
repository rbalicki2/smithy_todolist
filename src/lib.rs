#![feature(
  proc_macro_hygiene,
  slice_patterns,
  custom_attribute,
  extern_crate_item_prelude
)]

use js_sys::{
  global,
  Object,
};
use smithy::{
  self,
  smd,
};
use wasm_bindgen::prelude::*;
use web_sys::{
  Document,
  Element,
  Window,
};

fn get_window() -> Window {
  unsafe { std::mem::transmute::<Object, Window>(global()) }
}

fn get_document() -> Document {
  get_window().document().unwrap()
}

#[wasm_bindgen]
pub fn start(div_id: String) {
  let doc: Document = get_document();
  let root_element: Element = doc.get_element_by_id(&div_id).unwrap();

  let app = smd!(<div>
    This is a great place to start building a Smithy project!
  </div>);
  smithy::mount(Box::new(app), root_element);
}
