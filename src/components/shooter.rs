use specs::{Component, VecStorage};
use ggez::nalgebra::{Point2};

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct ShooterComponent {
  pub originOffset: Point2<f32>,
  pub cooldown: isize,
  pub rof: isize,
  pub speed: f32,
  pub direction: f32,
}