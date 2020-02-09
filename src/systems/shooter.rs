use specs::{System, ReadStorage, WriteStorage, Read, LazyUpdate, Entities};

use crate::components::*;

pub struct ShooterSystem;

impl<'a> System<'a> for ShooterSystem {
  type SystemData = (
    WriteStorage<'a, ShooterComponent>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, RotationComponent>,
    Read<'a, LazyUpdate>,
    Entities<'a>,
  );
  fn run(&mut self, (mut shooters, positions, rotations, updater, entities): Self::SystemData) {
    use specs::Join;
    let count = (&shooters, &positions, &rotations).join().count();
    for (shooter, position, rotation) in (&mut shooters, &positions, &rotations).join() {
      if (shooter.cooldown <= 0){
        let entity = entities.create();
        updater.insert(entity, Position {
          x: position.x,
          y: position.y
        });
        updater.insert(entity, RotationComponent(rotation.0));
        updater.insert(entity, ViewComponent::new(Views::Bullet));
        shooter.cooldown = shooter.rof.clone();
      } else {
        shooter.cooldown -= 1;
      }
    }
  }
}