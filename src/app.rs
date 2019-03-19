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
  types::Component,
};

pub fn render(mut app_state: AppState) -> impl smithy::types::Component {
  smd!(
    on_hash_change={|_| {
      app_state.handle_hash_change();
    }};
    {
      match app_state.current_page {
        Page::Home => {
          home::render_home_page(&app_state.todo_lists)
        },
        Page::TodoListDetail(id) => {
          detail_view::render_detail_view_page()
        },
      }
    }
  )
}
