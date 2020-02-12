/**
 * TODO:
 * - remove to removeWhenOutside
 * - check bounds respecting entity dimensions
 **/

use specs::{System, ReadStorage, WriteStorage, Entities};

use crate::components::*;

pub struct OutOfScreenRemover;

impl<'a> System<'a> for OutOfScreenRemover {
  type SystemData = (
    WriteStorage<'a, RemoveWhenOutOfScreen>,
    ReadStorage<'a, Position>,
    Entities<'a>,
  );
  fn run(&mut self, (ross, positions, entities): Self::SystemData) {
    use specs::Join;
    for (_ros, position, entity) in (&ross, &positions, &entities).join() {
      if position.y< -1000.0 || position.x< -1000.0 || position.y>3000.0 || position.x>3000.0{
        entities.delete(entity).unwrap();
      }
    }
  }
}