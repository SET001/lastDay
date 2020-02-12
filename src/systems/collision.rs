use specs::{System, ReadStorage, WriteStorage, Entities, Entity};
use ggez::nalgebra::{distance, Point2};
use crate::components::*;

struct CompGroup<'a>{
  // collision: &'a mut CollisionComponent,
  position: &'a Position,
  entitiy: Entity,
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
        position
      })
    }

    for(collision, position, entity) in (&mut collisions, &positions, &entities).join() {
      collision.collisions.clear();
      for group in &groupList{
        if entity != group.entitiy {
          let dist = distance(
            &Point2::new(position.x, position.y),
            &Point2::new(group.position.x, group.position.y),
          );
          if dist<collision.radius {
            collision.collisions.push(entity);
          }
        }
      }
    }
  }
}