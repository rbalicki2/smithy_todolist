use js_sys::{
  global,
  Object,
};

use web_sys::{
  Document,
  Element,
  Window,
};

pub fn get_window() -> Window {
  unsafe { std::mem::transmute::<Object, Window>(global()) }
}

pub fn get_document() -> Document {
  get_window().document().unwrap()
}
