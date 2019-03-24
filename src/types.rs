use crate::util::get_window;
use serde_derive::{
  Deserialize,
  Serialize,
};
use smithy::types::UnwrappedPromise;
use std::{
  cell::RefCell,
  collections::HashMap,
  rc::Rc,
};

pub type TodoListId = usize;

pub type TodoListHash = HashMap<TodoListId, TodoList>;
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoLists(TodoListHash);

impl std::ops::Deref for TodoLists {
  type Target = TodoListHash;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::ops::DerefMut for TodoLists {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl TodoLists {
  pub fn get_next_id(&self) -> TodoListId {
    if let Some(id) = self.keys().max() {
      id + 1
    } else {
      0
    }
  }

  pub fn new() -> TodoLists {
    TodoLists(HashMap::new())
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Showing {
  All,
  Complete,
  Incomplete,
}

impl Showing {
  pub fn filter(&self, todo_item: &TodoItem) -> bool {
    match self {
      Showing::All => true,
      Showing::Complete => todo_item.completed,
      Showing::Incomplete => !todo_item.completed,
    }
  }
}

pub type Ref = Option<web_sys::HtmlElement>;
pub type InputString = Rc<RefCell<String>>;

#[derive(Debug)]
pub enum Page {
  Home((Ref, InputString)),
  TodoListDetail((TodoListId, Ref, InputString, Showing)),
}

impl Page {
  pub fn handle_hash_change(&mut self) {
    if let Some(todo_list_id) = get_current_todo_list_id_from_hash() {
      *self = Page::TodoListDetail((
        todo_list_id,
        None,
        Rc::new(RefCell::new("".to_string())),
        Showing::All,
      ));
    } else {
      *self = Page::Home((None, Rc::new(RefCell::new("".into()))));
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
  pub completed: bool,
  pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
  pub name: String,
  pub items: Vec<TodoItem>,
}

impl TodoList {
  pub fn new(name: String) -> TodoList {
    TodoList {
      name,
      items: vec![],
    }
  }
}

pub type UnwrappedTodoListsApiRequest = UnwrappedPromise<TodoLists, ()>;

pub struct AppState {
  pub current_page: Page,
  pub todo_lists_api_request: UnwrappedTodoListsApiRequest,
}

impl AppState {
  pub fn new() -> AppState {
    // TODO should I combine these steps?
    let mut current_page = Page::Home((None, Rc::new(RefCell::new("".into()))));
    current_page.handle_hash_change();
    let app_state = AppState {
      current_page,
      todo_lists_api_request: smithy::unwrapped_promise_from_future(crate::api::fetch_todo_lists()),
    };
    app_state
  }
}

fn get_current_todo_list_id_from_hash() -> Option<TodoListId> {
  get_window()
    .location()
    .hash()
    .ok()
    .map(|hash_with_hash| hash_with_hash.chars().skip(1).collect::<String>())
    .and_then(|hash| hash.parse::<TodoListId>().ok())
}
