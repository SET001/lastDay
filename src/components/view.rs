use specs::{Component, VecStorage};
use ggez::graphics;

#[derive(Debug)]
pub enum Views{
  Human,
  Zombie,
  Bullet,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct ViewComponent{ 
  pub viewType: Views,
  pub meshes: Vec<Box<graphics::Drawable + Send + Sync>>
}
impl ViewComponent{
  pub fn new(viewType: Views)->ViewComponent{
    ViewComponent{
      viewType,
      meshes: Vec::new(),
    }
  }
}