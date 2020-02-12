use specs::{System, ReadStorage, WriteStorage, Entities};

use crate::components::*;
use rand::Rng;
pub struct DamageOnCollide;

impl<'a> System<'a> for DamageOnCollide {
  type SystemData = (
    ReadStorage<'a, DamageOnCollideComponent>,
    ReadStorage<'a, CollisionComponent>,
    Entities<'a>
  );
  fn run(&mut self, (damages, collisions, entities): Self::SystemData) {
    use specs::Join;
    let mut rng = rand::thread_rng();
    let round = rng.gen_range(0, 9999);
    for (damage, collision, entity) in (&damages, &collisions, &entities).join() {
      for collided_entity in &collision.collisions{
        // println!("{}: Entity {} was damaged by entity {}", round, entity.id(), collided_entity.id());
        entities.delete(*collided_entity).unwrap();
        entities.delete(entity).unwrap();
      }
    }
  }
}