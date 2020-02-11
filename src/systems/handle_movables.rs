use super::system_prelude::*;

#[derive(Default)]
pub struct HandleMovablesSystem;

impl<'a> System<'a> for HandleMovablesSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, MaxMovementVelocity>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut movables,
            mut velocities,
            max_movement_velocities,
            loadables,
            loadeds,
        ): Self::SystemData,
    ) {
        for (_, movable, velocity, max_velocity_opt) in (
            &entities,
            &mut movables,
            &mut velocities,
            max_movement_velocities.maybe(),
        )
            .join()
            .filter(|(entity, _, _, _)| {
                is_entity_loaded(*entity, &loadables, &loadeds)
            })
        {
            for action in movable.drain_actions() {
                match action {
                    MoveAction::Walk(axis, spd) => {
                        if let Some(max) = max_velocity_opt
                            .and_then(|max_velocity| max_velocity.get(&axis))
                        {
                            velocity.increase_with_max(&axis, spd, max);
                        } else {
                            velocity.increase(&axis, spd);
                        }
                    }
                    MoveAction::Jump(strength) => {
                        velocity.increase(&Axis::Y, strength);
                    }
                }
            }
        }
    }
}
