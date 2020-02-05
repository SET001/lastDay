use specs::{System, ReadStorage, WriteStorage};

use crate::components::{Position, Target};

pub struct LinearMovement;

impl<'a> System<'a> for LinearMovement {
  type SystemData = (
    WriteStorage<'a, Position>,
    ReadStorage<'a, Target>
  );
  fn run(&mut self, (mut position, target): Self::SystemData) {
    use specs::Join;
    for (position, _target) in (&mut position, &target).join() {
      position.x += 1.0;
    }
  }
}