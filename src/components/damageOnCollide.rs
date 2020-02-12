use specs::{Component, VecStorage, Entity};
use ggez::nalgebra::{Point2};

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct DamageOnCollideComponent (pub f32);