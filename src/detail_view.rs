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
  rc::Rc,
};

pub fn render_detail_view_page<'a>(
  todo_lists: &'a mut TodoLists,
  id: TodoListId,
  input_dom_ref: &'a mut Option<web_sys::HtmlElement>,
  text_value: &'a mut Rc<RefCell<String>>,
) -> SmithyComponent<'a> {
  smd!(
    <a href="#">Back to list</a>
    {
      match todo_lists.get_mut(&id) {
        Some(todo_list) => render_item_view(todo_list, input_dom_ref, text_value),
        None => smd!(no todolist with this id),
      }
    }
  )
}

pub fn render_item_view<'a>(
  todo_list: &'a mut TodoList,
  input_dom_ref: &'a mut Option<web_sys::HtmlElement>,
  input_text: &'a Rc<RefCell<String>>,
) -> SmithyComponent<'a> {
  let todo_list = Rc::new(RefCell::new(todo_list));
  let todo_list_2 = todo_list.clone();

  let input_text_2 = input_text.clone();
  let mut input = crate::input::render_input(
    input_text,
    |x| if x == "foo" { "bar".to_string() } else { x },
    move |description| {
      // on_enter
      *input_text_2.borrow_mut() = "".into();
      todo_list_2.borrow_mut().items.push(TodoItem {
        completed: false,
        description,
      });
    },
    input_dom_ref,
  );
  smd!(
    <h1>{ &todo_list.borrow().name }</h1>
    <ul>
      {
        todo_list.borrow().items.iter().map(|todo_item| smd!(<li>
          { if todo_item.completed { "x" } else { " " } }
          { &todo_item.description }
        </li>)).collect::<Vec<SmithyComponent>>()
      }
    </ul>
    { &mut input }
  )
}
