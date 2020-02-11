use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct ZombieSpawnerComponent {
  pub cooldown: f32,
  pub spawnRate: f32,
  pub radius: f32,
  pub spawnInitially: isize,
  pub initiallySpawned: bool,
}