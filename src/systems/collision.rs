use specs::{System, ReadStorage, WriteStorage, Entities, Entity};
use ggez::nalgebra::{distance, Point2};
use crate::components::*;

struct CompGroup<'a>{
  // collision: &'a mut CollisionComponent,
  position: &'a Position,
  entitiy: Entity,
  center: Point2<f32>,
  radius: f32,
  rotation: f32,
}

fn getPoint(position: &Point2<f32>, offset: &Point2<f32>, rotation: f32, scale: f32)->Point2<f32>{
  let dist = distance(
    &Point2::new(offset.x, offset.y),
    &Point2::new(0.0, 0.0)
  );
  let angle = (offset.y).atan2(offset.x);

  Point2::new(
    position.x+dist*(rotation+angle).cos()*scale,
    position.y+dist*(rotation+angle).sin()*scale
  )
}

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
  type SystemData = (
    WriteStorage<'a, CollisionComponent>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, RotationComponent>,
    Entities<'a>
  );

  fn run(&mut self, (mut collisions, positions, rotations, entities): Self::SystemData) {
    use specs::Join;
    
    let mut groupList:Vec<CompGroup> = Vec::new();
    for(_collision, position, rotation, entitiy) in (&mut collisions, &positions, &rotations, &entities).join() {
      groupList.push(CompGroup{
        entitiy,
        position,
        center: Point2::new(
          _collision.center.x,
          _collision.center.y,
        ),
        radius: _collision.radius.clone(),
        rotation: rotation.0
      })
    }

    for(collision, position, rotation, entity) in (&mut collisions, &positions, &rotations, &entities).join() {
      let scale = 0.3;
      collision.collisions.clear();
      for group in &groupList{
        if entity != group.entitiy {
          
          let dist = distance(
            &getPoint(
              &Point2::new(position.x, position.y),
              &Point2::new(collision.center.x, collision.center.y),
              rotation.0,
              scale
            ),
            &getPoint(
              &Point2::new(
                group.position.x,
                group.position.y),
              &Point2::new(
                group.center.x,
                group.center.y),
              group.rotation,
              scale
            )
          );
          // println!("distance {}", dist);
          let minDist = (collision.radius+group.radius)*scale;
          if dist<minDist {
            // println!("collision");
            collision.collisions.push(group.entitiy);
          }
        }
      }
    }
  }
}