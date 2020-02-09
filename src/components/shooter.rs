use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct ShooterComponent {
  pub cooldown: isize,
  pub rof: isize,
  pub speed: f32,
  pub direction: f32,
}