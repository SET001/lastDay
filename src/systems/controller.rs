use specs::{System, ReadStorage, Read, WriteStorage, Entities, LazyUpdate};
use std::f32::consts::PI;
use ggez::nalgebra::{Point2};

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
            originOffset: Point2::new(300.0, 155.0)
          }),
          _ => ()
        }

        
      } else {
        updater.remove::<ShooterComponent>(entity);
      }
    }
  }
}
