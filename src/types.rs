use crate::util::get_window;

type TodoListId = usize;

#[derive(Debug)]
pub enum Page {
  Home,
  TodoListDetail(TodoListId),
}

impl Page {
  pub fn to_class_name(&self) -> &'static str {
    match self {
      Page::Home => "home-page-visible",
      Page::TodoListDetail(_) => "todo-list-detail-page-visible",
    }
  }
}

// TODO wrap these id's in a newtype
type TodoItemId = usize;
#[derive(Debug, Clone)]
pub struct TodoItem {
  todo_item_id: TodoItemId,
  completed: bool,
  description: String,
}

#[derive(Debug, Clone)]
pub struct TodoList {
  pub name: String,
  pub todo_list_id: TodoListId,
  pub todo_items: Vec<TodoItem>,
}

#[derive(Debug)]
pub struct AppState {
  pub current_page: Page,
  pub todo_lists: Vec<TodoList>,
}

impl AppState {
  pub fn new() -> AppState {
    // TODO should I combine these steps?
    let mut app_state = AppState {
      current_page: Page::Home,
      todo_lists: vec![
        TodoList {
          name: "Shopping".into(),
          todo_list_id: 0,
          todo_items: vec![],
        },
        TodoList {
          name: "Housework".into(),
          todo_list_id: 1,
          todo_items: vec![],
        },
      ],
    };
    app_state.handle_hash_change();
    app_state
  }

  pub fn handle_hash_change(&mut self) {
    if let Some(todo_list_id) = get_current_todo_list_id_from_hash() {
      self.current_page = Page::TodoListDetail(todo_list_id);
    } else {
      self.current_page = Page::Home;
    }
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
