use super::system_prelude::*;

#[derive(Default)]
pub struct HandleMovablesSystem;

impl<'a> System<'a> for HandleMovablesSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, BaseFriction>,
        WriteStorage<'a, Gravity>,
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
            mut gravities,
            max_movement_velocities,
            loadables,
            loadeds,
        ): Self::SystemData,
    ) {
        for (
            _,
            movable,
            velocity,
            max_velocity_opt,
            mut base_friction_opt,
            mut gravity_opt,
        ) in (
            &entities,
            &mut movables,
            &mut velocities,
            max_movement_velocities.maybe(),
            (&mut base_frictions).maybe(),
            (&mut gravities).maybe(),
        )
            .join()
            .filter(|(entity, _, _, _, _, _)| {
                is_entity_loaded(*entity, &loadables, &loadeds)
            })
        {
            let mut friction_enabled = (true, true);
            let maybe_set_gravity = |gravity_opt: &mut Option<&mut Gravity>,
                                     gravity_strength: &(
                Option<f32>,
                Option<f32>,
            )| {
                if let Some(gravity_comp) = gravity_opt {
                    for axis in Axis::iter() {
                        if let Some(grav) = gravity_strength.by_axis(&axis) {
                            gravity_comp.set(&axis, *grav);
                        }
                    }
                }
            };

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

                    MoveAction::Jump { strength, gravity } => {
                        velocity.increase(&Axis::Y, strength);
                        maybe_set_gravity(&mut gravity_opt, dbg!(&gravity));
                    }

                    MoveAction::KillJump {
                        strength,
                        gravity,
                        min_velocity,
                    } => {
                        let vel = velocity.y;
                        if vel > min_velocity {
                            let decreased = (vel + strength).max(min_velocity);
                            velocity.set(&Axis::Y, decreased);
                        }
                        maybe_set_gravity(&mut gravity_opt, dbg!(&gravity));
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
