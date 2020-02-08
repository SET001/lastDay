use specs::{Component, VecStorage};
use ggez::graphics;

#[derive(Debug)]
pub enum Views{
  Human,
  Zombie
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ViewComponent{ 
  pub viewType: Views,
  pub meshes: Vec<graphics::Image>
}