use smithy::{
  smd,
  types::SmithyComponent,
};

pub fn render_home<'a>(mut transition_to: impl FnMut(usize) + 'a) -> SmithyComponent<'a> {
  smd!(
    <div
      on_click={|_| transition_to(3)}
    >
      home page byah
    </div>
  )
}
