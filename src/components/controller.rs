use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ControllerComponent{
  pub movingLeft: bool,
  pub movingRight: bool,
  pub movingUp: bool,
  pub movingDown: bool,
}