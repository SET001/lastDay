use specs::{Component, VecStorage, Entity};

#[derive(Component, Debug)]
#[storage(VecStorage)]

pub struct TargetComponent (pub Entity);