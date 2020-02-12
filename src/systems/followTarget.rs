use specs::{System, WriteStorage, ReadStorage, Read, LazyUpdate, Entities};

use crate::components::*;
pub struct FollowTarget;

impl<'a> System<'a> for FollowTarget {
  type SystemData = (
    ReadStorage<'a, Position>,
    WriteStorage<'a, RotationComponent>,
    ReadStorage<'a, TargetComponent>,
    WriteStorage<'a, LinearMovementComponent>,
    Entities<'a>,
    Read<'a, LazyUpdate>,
  );

  
  fn run(&mut self, (positions, mut rotations, targets, mut movements, entities, updater): Self::SystemData) {
    use specs::Join;
    for (position, rotation, target, entity, ()) in (&positions, &mut rotations, &targets, &entities, !&movements).join() {
      if let Some(targetPosition) = positions.get(target.0) {
        let direction = (targetPosition.y - position.y).atan2(targetPosition.x - position.x);
        rotation.0 = direction; 
        updater.insert(entity, LinearMovementComponent{
          direction,
          speed: 1.0  //  TODO: do not hardcode speed here
        })
      }
    }
    for (position, rotation, target, movement) in (&positions, &mut rotations, &targets, &mut movements).join() {
      if let Some(targetPosition) = positions.get(target.0) {
        let direction = (targetPosition.y - position.y).atan2(targetPosition.x - position.x);
        rotation.0 = direction; 
        movement.direction = direction;
      }
    }
  }
}