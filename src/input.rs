use smithy::{
  smd,
  types::SmithyComponent,
};
use std::{
  cell::RefCell,
  rc::Rc,
};
use wasm_bindgen::JsCast;
use web_sys::{
  HtmlInputElement,
  InputEvent,
  KeyboardEvent,
};

const ENTER_KEY: u32 = 13;

// this needs to be built with a builder pattern or something
pub fn render_input<'a>(
  value: &'a Rc<RefCell<String>>,
  // should be called... parse? format?
  transformer: impl Fn(String) -> String + 'a,
  mut on_enter: impl FnMut(String) + 'a,
  input_dom_ref: &'a mut Option<web_sys::HtmlElement>,
) -> SmithyComponent<'a> {
  smd!(
    post_render={|| {
      if let Some(el) = &input_dom_ref {
        let el: &web_sys::HtmlInputElement = el.unchecked_ref();
        el.set_value(&*value.borrow());
      }
    }};
    <input
      value={(&*value.borrow()).to_string()}
      class="form-control"
      ref={input_dom_ref}
      on_input={|e: &InputEvent| {
        let target = e.target().unwrap();
        let target: web_sys::HtmlInputElement = target.unchecked_into();
        let new_val = transformer(target.value());

        let mut value = value.borrow_mut();
        *value = new_val;
      }}
      on_key_down={|e: &KeyboardEvent| {
        if e.key_code() == ENTER_KEY {
          let input: HtmlInputElement = e.target().unwrap().unchecked_into();
          let text = input.value();
          on_enter(text);
        }
      }}
    />
  )
}
