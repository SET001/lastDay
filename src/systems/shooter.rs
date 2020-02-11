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
    for (shooter, position, rotation) in (&mut shooters, &positions, &rotations).join() {
      if shooter.cooldown <= 0 {
        let bullet = entities.create();
        updater.insert(bullet, Position {
          x: position.x,
          y: position.y
        });
        updater.insert(bullet, RotationComponent(rotation.0));
        updater.insert(bullet, ViewComponent::new(Views::Bullet));
        updater.insert(bullet, LinearMovementComponent{
          direction: rotation.0,
          speed: shooter.speed
        });
        updater.insert(bullet, CollisionComponent::new(5.0));
        updater.insert(bullet, RemoveWhenOutOfScreen{});
        shooter.cooldown = shooter.rof.clone();
      } else {
        shooter.cooldown -= 1;
      }
    }
  }
}