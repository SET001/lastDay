use specs::{Component, VecStorage};

#[derive(Debug, PartialEq)]
pub enum Fractions{
  Zombies,
  Humans
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct FractionableComponent(pub Fractions);