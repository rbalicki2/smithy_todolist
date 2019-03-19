use crate::types::TodoList;
use smithy::{
  smd,
  types::{
    Component,
    SmithyComponent,
  },
};

pub fn render_home_page<'a>(todo_lists: &'a Vec<TodoList>) -> SmithyComponent<'a> {
  smd!(
    <h1>Todo Lists</h1>
    <ul>
      {
        todo_lists.iter().map(|todo_list| {
          smd!(<li>
            <a href={format!("#{}", todo_list.todo_list_id)}>{&todo_list.name}</a>
          </li>)
        }).collect::<Vec<SmithyComponent>>()
      }
    </ul>
  )
}
