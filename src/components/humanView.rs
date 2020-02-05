use specs::{Component, VecStorage};
use ggez::graphics;
use ggez::{Context};
use ggez::nalgebra::{Point2};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct HumanViewComponent{
  pub mesh: graphics::Mesh
}

impl HumanViewComponent{
  pub fn new(ctx: &mut Context) -> HumanViewComponent{
    HumanViewComponent {
      mesh: graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        Point2::new(0.0, 0.0),
        3.0,
        0.1,
        graphics::WHITE,
      ).unwrap()
    }
  }
}