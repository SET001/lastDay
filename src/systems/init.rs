use specs::{System, Read, LazyUpdate, Entities};
use crate::prefab::*;

pub struct InitSystem;

impl<'a> System<'a> for InitSystem{
  type SystemData = (
    Entities<'a>,
    Read<'a, LazyUpdate>
  );

  fn run(&mut self, (entities, updater): Self::SystemData){
    PrefabBuiler::background(&entities, &updater);
    PrefabBuiler::player(&entities, &updater);
    PrefabBuiler::spawner(&entities, &updater);
  }
}