use futures::Future;
use js_sys::Promise;
// TODO figure out why these imports are being marked as unused
use crate::types::TodoLists;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
  Request,
  RequestInit,
  RequestMode,
  Response,
};

const BASE_URL: &'static str = "https://smithy-todolists.herokuapp.com";

pub fn fetch_todo_lists() -> impl Future<Item = TodoLists, Error = ()> {
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let request = Request::new_with_str_and_init(BASE_URL, &opts).unwrap();

  let window = web_sys::window().unwrap();
  let request_promise = window.fetch_with_request(&request);

  let future = JsFuture::from(request_promise)
    .and_then(|resp_value| {
      let resp: Response = resp_value.unchecked_into();
      resp.json()
    })
    .and_then(|json_value: Promise| {
      // Convert this other `Promise` into a rust `Future`.
      JsFuture::from(json_value)
    })
    .map(|json| {
      // Use serde to parse the JSON into a struct.
      let todo_lists_result = json.into_serde();
      todo_lists_result.unwrap_or_else(|_| TodoLists::new())
    })
    .map_err(|_| ());
  future
}

pub fn save_todo_lists(todo_lists: &TodoLists) {
  let mut opts = RequestInit::new();
  opts.method("POST");
  opts.mode(RequestMode::Cors);

  let request = Request::new_with_str_and_init(BASE_URL, &opts).unwrap();
  request
    .headers()
    .set("Content-Type", "application/json")
    .unwrap();

  let mut request_init = RequestInit::new();
  request_init.body(Some(&wasm_bindgen::JsValue::from_str(
    &serde_json::to_string(todo_lists).unwrap(),
  )));

  let window = web_sys::window().unwrap();
  let _ = window.fetch_with_request_and_init(&request, &request_init);
}
