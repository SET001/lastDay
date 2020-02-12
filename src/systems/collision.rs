use specs::{System, ReadStorage, WriteStorage, Entities, Entity};
use ggez::nalgebra::{distance, Point2};
use crate::components::*;

struct CompGroup<'a>{
  // collision: &'a mut CollisionComponent,
  position: &'a Position,
  entitiy: Entity,
  center: Point2<f32>,
  radius: f32,
}

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
  type SystemData = (
    WriteStorage<'a, CollisionComponent>,
    ReadStorage<'a, Position>,
    Entities<'a>
  );

  fn run(&mut self, (mut collisions, positions, entities): Self::SystemData) {
    use specs::Join;
    
    let mut groupList:Vec<CompGroup> = Vec::new();
    for(_collision, position, entitiy) in (&mut collisions, &positions, &entities).join() {
      groupList.push(CompGroup{
        entitiy,
        position,
        center: Point2::new(
          _collision.center.x,
          _collision.center.y,
        ),
        radius: _collision.radius.clone()
      })
    }

    for(collision, position, entity) in (&mut collisions, &positions, &entities).join() {
      let scale = 0.3;
      collision.collisions.clear();
      for group in &groupList{
        if entity != group.entitiy {
          let dist = distance(
            &Point2::new(position.x+collision.center.x*scale, position.y+collision.center.y*scale),
            &Point2::new(group.position.x+group.center.x*scale, group.position.y+group.center.y*scale),
          );
          let minDist = collision.radius+group.radius;
          if dist<minDist*scale {
            collision.collisions.push(group.entitiy);
          }
        }
      }
    }
  }
}