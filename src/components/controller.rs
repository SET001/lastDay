use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct ControllerComponent{
  pub movingLeft: bool,
  pub movingRight: bool,
  pub movingForward: bool,
  pub movingBackward: bool,
  pub isFiring: bool,
}