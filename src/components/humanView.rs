use specs::{Component, VecStorage};
use ggez::graphics;
use ggez::{Context};
use ggez::nalgebra::{Point2};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct HumanViewComponent{
  pub mesh: graphics::Image
}

impl HumanViewComponent{
  pub fn new(ctx: &mut Context) -> HumanViewComponent{
    HumanViewComponent {
      mesh: graphics::Image::new(ctx, "/survivor-idle_rifle_0.png").unwrap()
    }
  }
}