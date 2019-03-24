use crate::types::{
  InputString,
  Ref,
  TodoList,
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

pub fn render_home_page<'a>(
  todo_lists: &'a mut TodoLists,
  (input_dom_ref, input_text): &'a mut (Ref, InputString),
) -> SmithyComponent<'a> {
  let todo_lists = Rc::new(RefCell::new(todo_lists));
  let todo_lists_2 = todo_lists.clone();
  let input_text_2 = input_text.clone();

  let mut input = crate::input::render_input(
    input_text,
    |x| if x == "foo" { "bar".to_string() } else { x },
    move |new_todo_list_title| {
      // on_enter
      *input_text_2.borrow_mut() = "".into();
      let mut todo_lists_2 = todo_lists_2.borrow_mut();
      let id = todo_lists_2.get_next_id();
      todo_lists_2.insert(id, TodoList::new(new_todo_list_title));
    },
    input_dom_ref,
  );

  smd!(
    <h1>Todo Lists</h1>
    <ul>
      {
        todo_lists.borrow().iter().map(|(id, todo_list)| {
          smd!(<li>
            <a href={format!("#{}", id)}>{&todo_list.name}</a>
          </li>)
        }).collect::<Vec<SmithyComponent>>()
      }
    </ul>
    { &mut input }
  )
}
