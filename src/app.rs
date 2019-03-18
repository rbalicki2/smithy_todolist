use crate::{
  home,
  types::AppState,
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
    <style>{r"
      .home-page, .todo-list-detail-page {
        display: none;
      }
      .home-page-visible .home-page, .todo-list-detail-page-visible .todo-list-detail-page {
        display: block;
      }
    "}</style>
    <div class={app_state.get_current_page().to_class_name()}>
      <div class="home-page">
        { home::render_home(|todo_list_id: usize| app_state.transition_to(todo_list_id)) }
      </div>
      <div class="todo-list-detail-page">
        todo list detail page
      </div>
    </div>
  )
}
