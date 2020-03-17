use super::system_prelude::*;

#[derive(Default)]
pub struct HandleMovablesSystem;

impl<'a> System<'a> for HandleMovablesSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, BaseFriction>,
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
            mut base_frictions,
            max_movement_velocities,
            loadables,
            loadeds,
        ): Self::SystemData,
    ) {
        for (_, movable, velocity, max_velocity_opt, mut base_friction_opt) in (
            &entities,
            &mut movables,
            &mut velocities,
            max_movement_velocities.maybe(),
            (&mut base_frictions).maybe(),
        )
            .join()
            .filter(|(entity, _, _, _, _)| {
                is_entity_loaded(*entity, &loadables, &loadeds)
            })
        {
            let mut friction_enabled = (true, true);

            for action in movable.drain_actions() {
                match action {
                    MoveAction::Walk { axis, speed } => {
                        if let Some(max) = max_velocity_opt
                            .and_then(|max_velocity| max_velocity.get(&axis))
                        {
                            velocity.increase_with_max(&axis, speed, max);
                        } else {
                            velocity.increase(&axis, speed);
                        }

                        if base_friction_opt.is_some() {
                            *(&mut friction_enabled).by_axis(&axis) =
                                velocity.get(&axis).signum() != speed.signum();
                        }
                    }

                    MoveAction::Jump { strength } => {
                        velocity.set(&Axis::Y, strength);
                    }

                    MoveAction::KillJump {
                        strength,
                        min_velocity,
                    } => {
                        let vel = velocity.y;
                        if vel > min_velocity {
                            let decreased = (vel + strength).max(min_velocity);
                            velocity.set(&Axis::Y, decreased);
                        }
                    }

                    MoveAction::WallJump { strength } => {
                        for axis in Axis::iter() {
                            velocity.set(&axis, strength.by_axis(&axis));
                        }
                    }

                    MoveAction::WallSlide {
                        velocity: slide_vel,
                    } => {
                        if velocity.get(&Axis::Y) < slide_vel {
                            velocity.set(&Axis::Y, slide_vel);
                        }
                    }

                    MoveAction::AddVelocity {
                        velocity: add_velocity,
                    } => {
                        for axis in Axis::iter() {
                            if let Some(vel) = add_velocity.by_axis(&axis) {
                                if let Some(max) = max_velocity_opt
                                    .and_then(|max_vel| max_vel.get(&axis))
                                {
                                    velocity.increase_with_max(&axis, vel, max)
                                } else {
                                    velocity.increase(&axis, vel);
                                }
                            }
                        }
                    }

                    MoveAction::SetVelocity {
                        velocity: set_velocity,
                    } => {
                        for axis in Axis::iter() {
                            if let Some(vel) = set_velocity.by_axis(&axis) {
                                if let Some(max) = max_velocity_opt
                                    .and_then(|max_vel| max_vel.get(&axis))
                                {
                                    velocity.set_with_max(&axis, vel, max)
                                } else {
                                    velocity.set(&axis, vel);
                                }
                            }
                        }
                    }
                }
            }

            if let Some(base_friction) = base_friction_opt.as_mut() {
                for axis in Axis::iter() {
                    base_friction
                        .set_enabled(&axis, friction_enabled.by_axis(&axis));
                }
            }
        }
    }
}
