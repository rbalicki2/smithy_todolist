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

const BASE_URL: &'static str = "http://localhost:3000";

pub fn fetch_todo_lists() -> impl Future<Item = TodoLists, Error = ()> {
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let request = Request::new_with_str_and_init(BASE_URL, &opts).unwrap();

  request.headers().set("Accept", "application/json").unwrap();

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
      let post: TodoLists = json.into_serde().unwrap();
      post
    })
    .map_err(|_| ());
  future
}
