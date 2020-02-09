use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct LinearMovementComponent {
  pub direction: f32,
  pub speed: f32,
}