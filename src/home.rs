use crate::types::TodoLists;
use smithy::{
  smd,
  types::{
    Component,
    SmithyComponent,
  },
};

pub fn render_home_page<'a>(todo_lists: &'a TodoLists) -> SmithyComponent<'a> {
  smd!(
    <h1>Todo Lists</h1>
    <ul>
      {
        todo_lists.iter().map(|(id, todo_list)| {
          smd!(<li>
            <a href={format!("#{}", id)}>{&todo_list.name}</a>
          </li>)
        }).collect::<Vec<SmithyComponent>>()
      }
    </ul>
  )
}
