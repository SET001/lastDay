use specs::{System, ReadStorage, Read, WriteStorage, Entities, LazyUpdate};
use std::f32::consts::PI;

use crate::components::*;

pub struct ControllerSystem;

impl<'a> System<'a> for ControllerSystem {
  type SystemData = (
    ReadStorage<'a, ControllerComponent>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, RotationComponent>,
    ReadStorage<'a, ShooterComponent>,
    Read<'a, LazyUpdate>,
    Entities<'a>,
  );

  fn run(&mut self, (controllers, mut positions, rotations, shooters, updater, entities): Self::SystemData) {
    use specs::Join;
    let speed = 4.0;
    for (controller, position, rotation, entity) in (&controllers, &mut positions, &rotations, &entities).join(){
      if controller.isFiring {
        match shooters.get(entity){
          None => updater.insert(entity, ShooterComponent{
            cooldown: 0,
            rof: 20,
            speed: 10.0,
            direction: rotation.0,
          }),
          _ => ()
        }

        
      } else {
        updater.remove::<ShooterComponent>(entity);
      }

      if controller.movingRight {
        let angle = rotation.0 - PI / 180.0 * 90.0 ;
        position.x = position.x - speed * angle.cos();
        position.y = position.y - speed * angle.sin();
      }
      if controller.movingLeft {
        let angle = rotation.0 - PI / 180.0 * 90.0 ;
        position.x = position.x + speed * angle.cos();
        position.y = position.y + speed * angle.sin();
      }
      if controller.movingBackward {
        position.x = position.x - speed * rotation.0.cos();
        position.y = position.y - speed * rotation.0.sin();
      }
      if controller.movingForward {
        position.x = position.x + speed * rotation.0.cos();
        position.y = position.y + speed * rotation.0.sin();
      }
    }
  }
}
