use specs::{System, WriteStorage, ReadStorage, Read, LazyUpdate, Entities};

use crate::components::*;
pub struct FollowTarget;

impl<'a> System<'a> for FollowTarget {
  type SystemData = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, TargetComponent>,
    WriteStorage<'a, LinearMovementComponent>,
    Entities<'a>,
    Read<'a, LazyUpdate>,
  );

  
  fn run(&mut self, (positions, targets, mut movements, entities, updater): Self::SystemData) {
    use specs::Join;
    for (position, target, entity, ()) in (&positions, &targets, &entities, !&movements).join() {
      if let Some(targetPosition) = positions.get(target.0) {
        updater.insert(entity, LinearMovementComponent{
          direction: (targetPosition.y - position.y).atan2(targetPosition.x - position.x),
          speed: 1.0  //  TODO: do not hardcode speed here
        })
      }
    }
    for (position, target, entity, mut movement) in (&positions, &targets, &entities, &mut movements).join() {
      if let Some(targetPosition) = positions.get(target.0) {
        movement.direction = (targetPosition.y - position.y).atan2(targetPosition.x - position.x);
      }
    }
  }
}