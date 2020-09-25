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
            movable.add_action(MoveAction::AddVelocity {
                x: walker.x.map(|x| x * dt),
                y: walker.y.map(|y| y * dt),
            })
        }
    }
}
