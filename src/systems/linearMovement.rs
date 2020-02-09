use specs::{System, ReadStorage, WriteStorage};

use crate::components::*;

pub struct LinearMovement;

impl<'a> System<'a> for LinearMovement {
  type SystemData = (
    WriteStorage<'a, Position>,
    ReadStorage<'a, LinearMovementComponent>
  );
  fn run(&mut self, (mut position, movement): Self::SystemData) {
    use specs::Join;
    for (position, movement) in (&mut position, &movement).join() {
      position.x += movement.speed*movement.direction.cos();
      position.y += movement.speed*movement.direction.sin();
    }
  }
}