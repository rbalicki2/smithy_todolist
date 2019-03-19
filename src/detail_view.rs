use crate::types::TodoList;
use smithy::{
  smd,
  types::{
    Component,
    SmithyComponent,
  },
};

pub fn render_detail_view_page<'a>() -> SmithyComponent<'a> {
  smd!(<h1>
    Detail View
  </h1>
  <a href="#">Go Home</a>)
}
