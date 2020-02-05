use specs::{System, ReadStorage, Entities, Read, LazyUpdate};
use rand::Rng;

use crate::components::{Position, HumanViewComponent, Target};

pub struct ZombieSpawner;

impl<'a> System<'a> for ZombieSpawner {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, HumanViewComponent>,
    ReadStorage<'a, Position>,
    Read<'a, LazyUpdate>,
  );

  fn run(&mut self, (entities, views, positions, updater): Self::SystemData) {
    use specs::Join;
    let count = (&views, &positions).join().count();
    if count < 200000{
      let mut rng = rand::thread_rng();
      let enemy = entities.create();
      updater.insert(enemy, Position {
        x: rng.gen_range(0.0, 300.0),
        y: rng.gen_range(0.0, 300.0)
      });
      // updater.insert(enemy, HumanViewComponent::new());
      updater.insert(enemy, Target{
        x: 200.0,
        y: 200.0
      });
    }
  }
}
