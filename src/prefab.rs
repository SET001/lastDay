use specs::{World, WorldExt, Component, VecStorage, Entity, Builder, Entities, Read, LazyUpdate};
use crate::components::*;

pub struct PrefabBuiler<'a>{
  world: &'a mut World
}

impl<'a> PrefabBuiler<'a>{ 
  pub fn new(world: &'a mut World)->PrefabBuiler<'a>{
    PrefabBuiler{
      world
    }
  }

  pub fn background(entities: &Entities<'a>, updater: &Read<'a, LazyUpdate>) {
    let entity = entities.create();
    updater.insert(entity, Position{x: -100.0, y: 0.0});
    updater.insert(entity, RotationComponent(0.0));
    updater.insert(entity, ViewComponent::new (Views::Background));
  }

  pub fn player(entities: &Entities<'a>, updater: &Read<'a, LazyUpdate>) {
    let entity = entities.create();
    updater.insert(entity, Position{x: 0.0, y: 0.0});
    updater.insert(entity, RotationComponent(0.0));
    updater.insert(entity, ViewComponent::new (Views::Human));
    updater.insert(entity, Position{x: -100.0, y: 0.0});
    updater.insert(entity, ControllerComponent{
      movingLeft: false,
      movingRight: false,
      movingForward: false,
      movingBackward: false,
      isFiring: false,
    });
  }

  pub fn spawner(entities: &Entities<'a>, updater: &Read<'a, LazyUpdate>) {
    let entity = entities.create();
    updater.insert(entity, Position{x: 0.0, y: 0.0});
    updater.insert(entity, ZombieSpawnerComponent{
      radius: 300.0,
      spawnRate: dotenv!("zombieSpawner.spawnRate").parse::<f32>().unwrap(),
      cooldown: 0.0,
      spawnInitially: dotenv!("zombieSpawner.spawnInitially").parse::<isize>().unwrap(),
      initiallySpawned: false
    });
  }

  pub fn bullet(entities: &Entities<'a>, updater: &Read<'a, LazyUpdate>, x: f32, y: f32, direction: f32, speed: f32) {
    let entity = entities.create();
    updater.insert(entity, Position{x, y});
    updater.insert(entity, RotationComponent(0.0));
    updater.insert(entity, ViewComponent::new(Views::Bullet));
    updater.insert(entity, LinearMovementComponent{direction,speed});
    updater.insert(entity, CollisionComponent::new(5.0, 0.0, 0.0));
    updater.insert(entity, RemoveWhenOutOfScreen{});
    updater.insert(entity, DamageOnCollideComponent(100.0));
  }
}