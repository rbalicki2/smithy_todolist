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
use std::ops::Deref;

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
  smd!(
    <h1>{ &todo_list.name }</h1>
    <ul>
      {
        todo_list.items.iter().map(|todo_item| smd!(<li>
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
    <a
      on_click={|_| todo_list.items.push(TodoItem {
        todo_item_id: 0,
        completed: false,
        description: "foo".into()
      })}
    >
      Add todo list item
    </a>
  )
}
