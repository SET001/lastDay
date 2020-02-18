use specs::{System, ReadStorage, WriteStorage, Read, LazyUpdate, Entities};
use ggez::nalgebra::{distance, Point2};
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
    let scale = 0.3;
    for (shooter, position, rotation) in (&mut shooters, &positions, &rotations).join() {
      if shooter.cooldown <= 0 {
        let bullet = entities.create();
        let dist = distance(&Point2::new(0.0, 0.0), &shooter.originOffset);
        let angle = (shooter.originOffset.y).atan2(shooter.originOffset.x);
        let bulletPosition = Position {
          x: position.x+dist*(rotation.0+angle).cos()*scale,
          y: position.y-dist*(rotation.0+angle).sin()*scale
        };
        updater.insert(bullet, bulletPosition);
        updater.insert(bullet, RotationComponent(0.0));
        updater.insert(bullet, ViewComponent::new(Views::Bullet));
        updater.insert(bullet, LinearMovementComponent{
          direction: rotation.0,
          speed: shooter.speed
        });
        updater.insert(bullet, CollisionComponent::new(5.0, 0.0, 0.0));
        updater.insert(bullet, RemoveWhenOutOfScreen{});
        updater.insert(bullet, DamageOnCollideComponent(100.0));
        shooter.cooldown = shooter.rof.clone();
      } else {
        shooter.cooldown -= 1;
      }
    }
  }
}