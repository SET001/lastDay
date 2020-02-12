use specs::{System, ReadStorage, Read, LazyUpdate, Entities};

use crate::components::*;
pub struct TargetOnFraction;

impl<'a> System<'a> for TargetOnFraction {
  type SystemData = (
    ReadStorage<'a, FractionableComponent>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, TargetComponent>,
    ReadStorage<'a, TargetOnFractionsComponent>,
    Entities<'a>,
    Read<'a, LazyUpdate>,
  );
  fn run(&mut self, (fractionables, positions, targets, fractionsTargetables, entities, updater): Self::SystemData) {
    use specs::Join;
    for (position, fractionTargets, entity) in (&positions, &fractionsTargetables, &entities).join() {
      if let None = targets.get(entity) {
        //  TODO: sort them by distance to entity
        //  TODO: check if distance is withing discovery range
        let data = (&fractionables, &positions, &entities)
          .join()
          .filter(|&(fractionable, _, _)| fractionTargets.0.contains(&fractionable.0))
          .collect::<Vec<_>>();
        if data.len() > 0 {
          let (_, _, targetEntity) = data[0];
          updater.insert(entity, TargetComponent(targetEntity))
        }
      }
    }
  }
}