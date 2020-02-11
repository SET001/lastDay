use specs::Join;
use specs::{System, ReadStorage, WriteStorage, Entities, Read, LazyUpdate};
use rand::Rng;
use std::f32::consts::PI;

use crate::components::*;

pub struct ZombieSpawner;

impl<'a> System<'a> for ZombieSpawner {
  type SystemData = (
    WriteStorage<'a, ZombieSpawnerComponent>,
    ReadStorage<'a, Position>,
    Entities<'a>,
    Read<'a, LazyUpdate>,
  );

  fn run(&mut self, (mut spawners, positions, entities, updater): Self::SystemData) {
    let mut rng = rand::thread_rng();
    for (spawner, position) in (&mut spawners, &positions).join() {
      if spawner.cooldown <= 0.0 {
        let zombie = entities.create();
        let angle = rng.gen_range(-PI, PI);

        updater.insert(zombie, Position {
          x: position.x + spawner.radius*angle.cos(),
          y: position.y + spawner.radius*angle.sin()
        });
        updater.insert(zombie, RotationComponent(0.0));
        updater.insert(zombie, ViewComponent::new (Views::Zombie));
        updater.insert(zombie, CollisionComponent::new(50.0, 100.0, 120.0));
        spawner.cooldown = spawner.spawnRate.clone();
      } else {
        spawner.cooldown -= 1.0;
      }
    }
  }
}
