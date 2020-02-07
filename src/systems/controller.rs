use specs::{System, ReadStorage, WriteStorage, Entities, Read, LazyUpdate};
use std::f32::consts::PI;

use crate::components::*;

pub struct ControllerSystem;

impl<'a> System<'a> for ControllerSystem {
  type SystemData = (
    ReadStorage<'a, ControllerComponent>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, RotationComponent>,
  );

  fn run(&mut self, (controllers, mut positions, rotations): Self::SystemData) {
    use specs::Join;
    let speed = 2.0;
    for (controller, position, rotation) in (&controllers, &mut positions, &rotations).join(){
      if controller.movingRight {
        let angle = rotation.0 - PI / 180.0 * 90.0 ;
        position.x = position.x + speed * angle.cos();
        position.y = position.y + speed * angle.sin();
      //   position.x += 1.0;
                // rotation.0 + PI / 180.0 * 90.0,
      }
      if controller.movingLeft {
        let angle = rotation.0 - PI / 180.0 * 90.0 ;
        position.x = position.x - speed * angle.cos();
        position.y = position.y - speed * angle.sin();
      }
      if controller.movingBackward {
        position.x = position.x - speed * rotation.0.cos();
        position.y = position.y - speed * rotation.0.sin();
      }
      if controller.movingForward {
        position.x = position.x + speed * rotation.0.cos();
        position.y = position.y + speed * rotation.0.sin();
      }
    }
  }
}
