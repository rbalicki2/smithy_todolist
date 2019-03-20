use crate::types::{
  TodoItem,
  TodoList,
  TodoListId,
  TodoLists,
};
use smithy::{
  smd,
  types::{
    Component,
    SmithyComponent,
  },
};
use std::{
  cell::RefCell,
  ops::Deref,
  rc::Rc,
};

pub fn render_detail_view_page<'a>(
  mut todo_lists: &'a mut TodoLists,
  id: TodoListId,
) -> SmithyComponent<'a> {
  match todo_lists.get_mut(&id) {
    Some(todo_list) => render_item_view(todo_list, id),
    None => smd!(no todolist with this id),
  }
}

pub fn render_item_view<'a>(todo_list: &'a mut TodoList, id: TodoListId) -> SmithyComponent<'a> {
  let todo_list = Rc::new(RefCell::new(todo_list));
  let todo_list_2 = todo_list.clone();

  let mut text = Rc::new(RefCell::new("".to_string()));
  let text_value = text.clone();
  let mut input = crate::input::render_input(
    text,
    |x| x,
    move |description| {
      web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "desc = {}",
        description
      )));
      *text_value.borrow_mut() = "".into();
      todo_list_2.borrow_mut().items.push(TodoItem {
        todo_item_id: 0,
        completed: false,
        description,
      });
    },
  );
  smd!(
    <h1>{ &todo_list.borrow().name }</h1>
    <ul>
      {
        todo_list.borrow().items.iter().map(|todo_item| smd!(<li>
          {
            if todo_item.completed {
              "x"
            } else {
              " "
            }
          }
          { &todo_item.description }
        </li>)).collect::<Vec<SmithyComponent>>()
      }
    </ul>
    { &mut input }
  )
}
