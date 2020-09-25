use super::system_prelude::*;

#[derive(Default)]
pub struct HandleWalkersSystem;

impl<'a> System<'a> for HandleWalkersSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Walker>,
        WriteStorage<'a, Movable>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            walkers,
            mut movables,
            unloaded_store,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (_, walker, movable, _) in
            (&entities, &walkers, &mut movables, !&unloaded_store).join()
        {
            if let Some(x) = &walker.x {
                movable.add_action(MoveAction::Walk {
                    axis: Axis::X,
                    mult: x.num() * dt,
                })
            }
            if let Some(y) = &walker.y {
                movable.add_action(MoveAction::Walk {
                    axis: Axis::Y,
                    mult: y.num() * dt,
                })
            }
        }
    }
}
