use specs::{System, ReadStorage, WriteStorage, Entities, Read, LazyUpdate};
use rand::Rng;


use crate::components::*;

pub struct ControllerSystem;

impl<'a> System<'a> for ControllerSystem {
  type SystemData = (
    ReadStorage<'a, ControllerComponent>,
    WriteStorage<'a, Position>,
  );

  fn run(&mut self, (controllers, mut positions): Self::SystemData) {
    use specs::Join;
    for (controller, position) in (&controllers, &mut positions).join(){
      if controller.movingRight {
        position.x += 1.0;
      }
      if controller.movingLeft {
        position.x -= 1.0;
      }
      if controller.movingDown {
        position.y += 1.0;
      }
      if controller.movingUp {
        position.y -= 1.0;
      }
    }
  }
}
