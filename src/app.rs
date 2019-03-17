use crate::{
  types::{
    AppState,
    Page,
  },
  util,
};
use smithy::{
  smd,
  types::Component,
};
use std::{
  cell::RefCell,
  rc::Rc,
};

pub fn render(app_state: AppState) -> impl smithy::types::Component {
  let app_state = Rc::new(RefCell::new(app_state));

  smd!(
    on_hash_change={|_| {
      app_state.borrow_mut().handle_hash_change();
    }};
    {
      let app_state_2 = app_state.clone();
      match app_state.borrow().get_current_page() {
        Page::Home => smd!(<div
          on_click={|_| {
            app_state_2.borrow_mut().transition_to(1);
          }}
        >
          home page
        </div>),
        Page::UserDetail(user_id) => smd!(<div>detail view</div>),
      }
    }
  )
}
