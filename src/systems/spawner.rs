use specs::Join;
use specs::{System, ReadStorage, WriteStorage, Entities, Read, LazyUpdate};
use rand::Rng;
use rand::rngs::{ThreadRng};
// use rand::Rng::{ThreadRng};

use std::f32::consts::PI;

use crate::components::*;

fn spawn<'a>(
  spawner: &mut ZombieSpawnerComponent,
  position: &Position,
  entities: &Entities<'a>,
  updater: &Read<'a, LazyUpdate>,
  rng: &mut ThreadRng,
){
  let zombie = entities.create();
  let angle = rng.gen_range(-PI, PI);

  updater.insert(zombie, Position {
    x: position.x + spawner.radius*angle.cos(),
    y: position.y + spawner.radius*angle.sin()
  });
  updater.insert(zombie, RotationComponent(0.0));
  updater.insert(zombie, ViewComponent::new (Views::Zombie));
  updater.insert(zombie, CollisionComponent::new(50.0, 100.0, 120.0));}

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
    for (mut spawner, position) in (&mut spawners, &positions).join() {
      if spawner.spawnInitially>0 && !spawner.initiallySpawned {
        for _o in 0..spawner.spawnInitially {
          spawn(&mut spawner, &position, &entities, &updater, &mut rng);  
          spawner.initiallySpawned = true;
        }
      }
      if spawner.spawnRate > 0.0 {
        if spawner.cooldown <= 0.0 {
          spawn(&mut spawner, &position, &entities, &updater, &mut rng);
          spawner.cooldown = spawner.spawnRate.clone();
        } else {
          spawner.cooldown -= 1.0;
        }
      }
    }
  }
}
