use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct ShooterComponent {
  pub cooldown: isize,
  pub rof: isize
}