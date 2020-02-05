use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Target {
  pub x: f32,
  pub y: f32,
}

impl Component for Target {
  type Storage = VecStorage<Self>;
}