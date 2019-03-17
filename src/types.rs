use crate::util::get_window;

type UserId = usize;

#[derive(Debug)]
pub enum Page {
  Home,
  UserDetail(UserId),
}

#[derive(Debug)]
pub struct AppState {
  current_page: Page,
}

impl AppState {
  pub fn new() -> AppState {
    // TODO should I combine these?
    let mut app_state = AppState {
      current_page: Page::Home,
    };
    app_state.handle_hash_change();
    app_state
  }

  pub fn get_current_page(&self) -> &Page {
    &self.current_page
  }

  pub fn transition_to(&mut self, id: UserId) {
    *self = AppState {
      current_page: Page::UserDetail(id),
    };
  }

  pub fn handle_hash_change(&mut self) {
    if let Some(user_id) = get_current_user_id_from_hash() {
      self.current_page = Page::UserDetail(user_id);
    } else {
      self.current_page = Page::Home;
    }
  }
}

fn get_current_user_id_from_hash() -> Option<UserId> {
  get_window()
    .location()
    .hash()
    .ok()
    .map(|hash_with_hash| hash_with_hash.chars().skip(1).collect::<String>())
    .and_then(|hash| hash.parse::<UserId>().ok())
}
