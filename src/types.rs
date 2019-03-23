use crate::util::get_window;
use std::{
  cell::RefCell,
  collections::HashMap,
  rc::Rc,
};

pub type TodoListId = usize;

pub type TodoListHash = HashMap<TodoListId, TodoList>;
#[derive(Debug)]
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

#[derive(Debug)]
pub enum Page {
  Home,
  TodoListDetail(
    (
      TodoListId,
      Option<web_sys::HtmlElement>,
      Rc<RefCell<String>>,
    ),
  ),
}

impl Page {
  pub fn handle_hash_change(&mut self) {
    if let Some(todo_list_id) = get_current_todo_list_id_from_hash() {
      *self = Page::TodoListDetail((todo_list_id, None, Rc::new(RefCell::new("".to_string()))));
    } else {
      *self = Page::Home;
    }
  }
}

#[derive(Debug)]
pub struct TodoItem {
  pub completed: bool,
  pub description: String,
}

#[derive(Debug)]
pub struct TodoList {
  pub name: String,
  pub items: Vec<TodoItem>,
}

#[derive(Debug)]
pub struct AppState {
  pub current_page: Page,
  // pub todo_lists: Vec<TodoList>,
  pub todo_lists: TodoLists,
}

impl AppState {
  pub fn new() -> AppState {
    // TODO should I combine these steps?
    let mut current_page = Page::Home;
    current_page.handle_hash_change();
    let app_state = AppState {
      current_page,
      todo_lists: TodoLists({
        let mut map = HashMap::new();
        map.insert(
          0,
          TodoList {
            name: "Housework".into(),
            items: vec![TodoItem {
              completed: false,
              description: "Do the dishes".into(),
            }],
          },
        );
        map.insert(
          1,
          TodoList {
            name: "Programming".into(),
            items: vec![
              TodoItem {
                completed: true,
                description: "Learn Rust".into(),
              },
              TodoItem {
                completed: false,
                description: "Build a site with Smithy".into(),
              },
            ],
          },
        );
        map
      }),
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
