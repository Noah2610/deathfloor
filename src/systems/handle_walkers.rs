use super::system_prelude::*;

#[derive(Default)]
pub struct HandleWalkersSystem;

impl<'a> System<'a> for HandleWalkersSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Walker>,
        WriteStorage<'a, Movable>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            walkers,
            mut movables,
            loadables,
            loadeds,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (_, walker, movable) in (&entities, &walkers, &mut movables)
            .join()
            .filter(|(entity, _, _)| {
                is_entity_loaded(*entity, &loadables, &loadeds)
            })
        {
            movable.add_action(MoveAction::AddVelocity {
                x: walker.x.map(|x| x * dt),
                y: walker.y.map(|y| y * dt),
            })
        }
    }
}
