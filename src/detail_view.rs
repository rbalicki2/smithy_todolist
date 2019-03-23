use crate::types::{
  Showing,
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
  showing: &'a mut Showing,
) -> SmithyComponent<'a> {
  smd!(
    <a href="#">Back to list</a>
    {
      match todo_lists.get_mut(&id) {
        Some(todo_list) => render_item_view(todo_list, input_dom_ref, text_value, showing),
        None => smd!(no todolist with this id),
      }
    }
  )
}

pub fn render_item_view<'a>(
  todo_list: &'a mut TodoList,
  input_dom_ref: &'a mut Option<web_sys::HtmlElement>,
  input_text: &'a Rc<RefCell<String>>,
  showing: &'a mut Showing,
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
  let showing_clone = showing.clone();
  let get_button_class = move |new_showing: Showing| {
    if new_showing == showing_clone {
      "detail_view_button active"
    } else {
      "detail_view_button inactive"
    }
  };

  smd!(
    <style type="text/css">{r"
      .active {
        text-decoration: underline;
      }
      .detail_view_button {
        border: none;
        cursor: pointer;
      }
      .detail_view_button:focus {
        outline: 0;
      }
    "}</style>
    <h1>{ &todo_list.borrow().name }</h1>
    <button
      on_click={|_| *showing = Showing::Complete}
      class={get_button_class(Showing::Complete)}
    >
      Complete
    </button>
    <button
      on_click={|_| *showing = Showing::Incomplete}
      class={get_button_class(Showing::Incomplete)}
    >
      Incomplete
    </button>
    <button
      on_click={|_| *showing = Showing::All}
      class={get_button_class(Showing::All)}
    >
      All
    </button>
    <ul>
      {
        todo_list.borrow_mut().items.iter_mut()
        .filter(|item| showing.filter(item))
        .map(|todo_item|
          smd!(<li
            on_click={|_| todo_item.completed = !todo_item.completed}
            style={format!(r"
              cursor: pointer;
              user-select: none;
              font-style: {};
            ", if todo_item.completed { "italic" } else { "normal" })}
          >
            { &todo_item.description }
          </li>)
        ).collect::<Vec<SmithyComponent>>()
      }
    </ul>
    { &mut input }
  )
}
