use specs::{Component, VecStorage, Entity};
use super::fractionable::{Fractions};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TargetOnFractionsComponent(pub Vec<Fractions>);