use crate::types::TodoList;
use smithy::{
  smd,
  types::{
    Component,
    SmithyComponent,
  },
};

pub fn render_detail_view_page<'a>(todo_lists: &'a mut Vec<TodoList>) -> SmithyComponent<'a> {
  smd!(
    <h1>
      Detail View
    </h1>
    <a href="#">Go Home</a>
    <ul>
      {
        todo_lists.iter().map(|_| smd!(<li>item</li>)).collect::<Vec<SmithyComponent>>()
      }
    </ul>
  )
}
