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
      .home-page, .user-detail-page {
        display: none;
      }
      .home-page-visible .home-page {
        display: block;
      }
      .user-detail-page-visible .user-detail-page {
        display: block;
      }
    "}</style>
    <div class={app_state.get_current_page().to_class_name()}>
      <div class="home-page">
        { home::render_home(|user_id: usize| app_state.transition_to(user_id)) }
      </div>
      <div class="user-detail-page">
        user detail page
      </div>
    </div>
  )
}
