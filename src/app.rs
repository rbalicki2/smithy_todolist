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

pub fn render(app_state: AppState) -> impl smithy::types::Component {
  let AppState {
    mut current_page,
    todo_lists_api_request,
  } = app_state;
  smd!(
    on_hash_change={|_| {
      &mut current_page.handle_hash_change();
    }};
    {
      match (&mut *todo_lists_api_request.borrow_mut(), &mut current_page) {
        (PromiseState::Success(ref mut todo_lists), &mut Page::Home(ref mut home_info)) => smd!(
          { home::render_home_page(todo_lists, home_info) }
          <div><button on_click={|_| crate::api::save_todo_lists(todo_lists)}>
            Save
          </button></div>
        ),
        (PromiseState::Success(ref mut todo_lists), &mut Page::TodoListDetail((ref id, ref mut input_dom_ref, ref mut input_text, ref mut showing))) => {
          smd!(
            { detail_view::render_detail_view_page(todo_lists, *id, input_dom_ref, input_text, showing) }
            <div><button on_click={|_| crate::api::save_todo_lists(todo_lists)}>
              Save
            </button></div>
          )
        },
        (PromiseState::Pending, _) => smd!(Fetching),
        (PromiseState::Error(_), _) => smd!(Something went wrong fetching the todo lists! Is the server running?),
      }
    }
  )
}
