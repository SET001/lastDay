use specs::{Component, VecStorage, Entity};
use ggez::nalgebra::{Point2};

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct CollisionComponent {
  pub radius: f32,
  pub center: Point2<f32>,
  pub collisions: Vec<Entity>
}

impl CollisionComponent{
  pub fn new(radius: f32) -> CollisionComponent {
    CollisionComponent {
      radius,
      center: Point2::new(0.0, 0.0),
      collisions: Vec::new(),
    }
  }
}