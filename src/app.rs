use crate::{
  detail_view,
  home,
  types::{
    AppState,
    Page,
  },
};
use smithy::{
  smd,
  types::{
    Component,
    PromiseState,
  },
};

const BOOTSTRAP_URL: &'static str =
  "https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/css/bootstrap.min.css";

pub fn render(app_state: AppState) -> impl smithy::types::Component {
  let AppState {
    mut current_page,
    todo_lists_api_request,
  } = app_state;
  smd!(
    on_hash_change={|_| {
      &mut current_page.handle_hash_change();
    }};
    <link rel="stylesheet" type="text/css" href={BOOTSTRAP_URL} />
    <style type="text/css">{ r"
      @font-face {
        font-family: mainFont;
        src: url(https://d32dj4qqmd0v7v.cloudfront.net/fonts/MaisonNeue/MaisonNeue-Medium.woff2);
      }

      .save_button {
        margin-top: 5vw;
      }

      .center {
        display: flex;
        flex-direction: row;
        justify-content: center;
      }

      h1 {
        margin-bottom: 10px;
      }
    "}</style>
    <div class="container" style={r"
      width: 80vw;
      padding: 10vw;
      margin-top: 10vw;
      background-color: #FAFAFA;
      box-shadow: 0 30px 25px -25px rgba(7,22,30,.15), 0 0 30px 0 rgba(7,22,30,.1);
      font-family: mainFont;
    "}>
      {
        match (&mut *todo_lists_api_request.borrow_mut(), &mut current_page) {
          (PromiseState::Success(ref mut todo_lists), &mut Page::Home(ref mut home_info)) => smd!(
            { home::render_home_page(todo_lists, home_info) }
            <div class="save_button center">
              <button on_click={|_| crate::api::save_todo_lists(todo_lists)} class="btn btn-primary">
                Save Todo Lists
              </button>
            </div>
          ),
          (PromiseState::Success(ref mut todo_lists), &mut Page::TodoListDetail((ref id, ref mut input_dom_ref, ref mut input_text, ref mut showing))) => {
            smd!(
              { detail_view::render_detail_view_page(todo_lists, *id, input_dom_ref, input_text, showing) }
              <div class="save_button center">
                <button on_click={|_| crate::api::save_todo_lists(todo_lists)} class="btn btn-primary">
                  Save Todo Lists
                </button>
              </div>
            )
          },
          (PromiseState::Pending, _) => smd!(Fetching),
          (PromiseState::Error(_), _) => smd!(Something went wrong fetching the todo lists! Is the server running?),
        }
      }
    </div>
  )
}
