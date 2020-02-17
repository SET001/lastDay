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

impl ControllerComponent{
  pub fn new() -> ControllerComponent{
    ControllerComponent{
      movingLeft: false,
      movingRight: false,
      movingForward: false,
      movingBackward: false,
      isFiring: false,
    }
  }
}