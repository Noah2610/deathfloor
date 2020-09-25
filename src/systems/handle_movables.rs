use super::system_prelude::*;

#[derive(Default)]
pub struct HandleMovablesSystem;

impl<'a> System<'a> for HandleMovablesSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, BaseFriction>,
        ReadStorage<'a, MovementAcceleration>,
        ReadStorage<'a, MaxMovementVelocity>,
        ReadStorage<'a, Jumper>,
        ReadStorage<'a, WallJumper>,
        ReadStorage<'a, WallSlider>,
        WriteStorage<'a, SoundPlayer<SoundType>>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut movables,
            mut velocities,
            mut base_frictions,
            movement_acceleration_store,
            max_movement_velocities,
            jumper_store,
            wall_jumper_store,
            wall_slider_store,
            mut sound_player_store,
            unloaded_store,
        ): Self::SystemData,
    ) {
        for (
            _,
            movable,
            velocity,
            max_velocity_opt,
            mut base_friction_opt,
            movement_acceleration_opt,
            jumper_opt,
            wall_jumper_opt,
            wall_slider_opt,
            mut sound_player_opt,
            _,
        ) in (
            &entities,
            &mut movables,
            &mut velocities,
            max_movement_velocities.maybe(),
            (&mut base_frictions).maybe(),
            movement_acceleration_store.maybe(),
            jumper_store.maybe(),
            wall_jumper_store.maybe(),
            wall_slider_store.maybe(),
            (&mut sound_player_store).maybe(),
            !&unloaded_store,
        )
            .join()
        {
            let mut friction_enabled = (true, true);

            for action in movable.drain_actions() {
                match action {
                    MoveAction::AddVelocity { x, y } => {
                        let add_velocity = (x, y);
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

                    MoveAction::SetVelocity { x, y } => {
                        let set_velocity = (x, y);
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

                    MoveAction::Walk { axis, mult } => {
                        if let Some(speed) = movement_acceleration_opt
                            .and_then(|accel| accel.by_axis(&axis).as_ref())
                            .map(|accel| accel * mult)
                        {
                            if let Some(max) =
                                max_velocity_opt.and_then(|max_velocity| {
                                    max_velocity.get(&axis)
                                })
                            {
                                velocity.increase_with_max(&axis, speed, max);
                            } else {
                                velocity.increase(&axis, speed);
                            }

                            if base_friction_opt.is_some() {
                                *(&mut friction_enabled).by_axis(&axis) =
                                    velocity.get(&axis).signum()
                                        != speed.signum();
                            }
                        } else {
                            eprintln!(
                                "[WARNING]\n    Can't use MoveAction::Walk on \
                                 axis {} without MovementAcceleration having \
                                 that axis set.",
                                &axis,
                            );
                        }
                    }

                    MoveAction::Jump => {
                        if let Some(jumper) = jumper_opt {
                            for axis in Axis::iter() {
                                if let Some(strength) =
                                    jumper.strength.by_axis(&axis)
                                {
                                    velocity.set(&axis, *strength);
                                }
                            }
                            // TODO
                            if let Some(sound_player) =
                                sound_player_opt.as_mut()
                            {
                                sound_player.add_action(
                                    SoundAction::PlayWithVolume(
                                        SoundType::Jump,
                                        0.5,
                                    ),
                                );
                            }
                        } else {
                            eprintln!(
                                "[WARNING]\n    Can't use MoveAction::Jump \
                                 without Jumper component."
                            );
                        }
                    }

                    MoveAction::KillJump => {
                        if let Some(jumper) = jumper_opt {
                            for axis in Axis::iter() {
                                let vel = velocity.by_axis(&axis);
                                if let Some(kill_strength) = jumper
                                    .kill_strength
                                    .by_axis(&axis)
                                    .as_ref()
                                    .copied()
                                {
                                    let min_velocity = jumper
                                        .min_velocity
                                        .by_axis(&axis)
                                        .unwrap_or(0.0);
                                    if kill_strength > min_velocity {
                                        let decreased = (vel + kill_strength)
                                            .max(min_velocity);
                                        velocity.set(&Axis::Y, decreased);
                                    }
                                }
                            }
                        } else {
                            eprintln!(
                                "[WARNING]\n    Can't use \
                                 MoveAction::KillJump without Jumper \
                                 component."
                            );
                        }
                    }

                    MoveAction::WallJump { x_mult } => {
                        if let (Some(jumper), Some(wall_jumper)) =
                            (jumper_opt, wall_jumper_opt)
                        {
                            for axis in Axis::iter() {
                                if let Some(strength) =
                                    wall_jumper.strength.by_axis(&axis)
                                {
                                    let mut strength = *strength;
                                    if let Axis::X = &axis {
                                        strength = strength * x_mult;
                                    }
                                    velocity.set(&axis, strength);
                                }
                            }
                            // TODO
                            if let Some(sound_player) =
                                sound_player_opt.as_mut()
                            {
                                sound_player.add_action(
                                    SoundAction::PlayWithVolume(
                                        SoundType::Jump,
                                        0.5,
                                    ),
                                );
                            }
                        } else {
                            eprintln!(
                                "[WARNING]\n    Can't use \
                                 MoveAction::WallJump without Jumper or \
                                 WallJumper component."
                            );
                        }
                    }

                    MoveAction::WallSlide => {
                        if let Some(wall_slider) = wall_slider_opt {
                            if velocity.get(&Axis::Y)
                                < wall_slider.slide_velocity
                            {
                                velocity
                                    .set(&Axis::Y, wall_slider.slide_velocity);
                            }
                        } else {
                            eprintln!(
                                "[WARNING]\n    Can't use \
                                 MoveAction::WallSlide without WallSlider \
                                 component."
                            );
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
