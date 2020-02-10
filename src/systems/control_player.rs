use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadExpect<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, MovementData>,
        WriteStorage<'a, Velocity>,
        // WriteStorage<'a, DecreaseVelocity>, // TODO
        WriteStorage<'a, BaseFriction>,
    );

    fn run(
        &mut self,
        (
            time,
            input_manager,
            players,
            movement_data_store,
            mut velocities,
            // mut decr_velocities, // TODO
            mut base_frictions,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (
            _,
            player_movement_data,
            player_velocity,
            // player_decr_velocity, // TODO
            mut player_base_friction_opt,
        ) in (
            &players,
            &movement_data_store,
            &mut velocities,
            // &mut decr_velocities, // TODO
            (&mut base_frictions).maybe(),
        )
            .join()
        {
            Axis::for_each(|axis| {
                handle_move_on_axis(
                    axis,
                    dt,
                    &input_manager,
                    player_movement_data,
                    player_velocity,
                    // player_decr_velocity, // TODO
                    &mut player_base_friction_opt,
                );
            });
        }
    }
}

fn handle_move_on_axis(
    axis: Axis,
    dt: f32,
    input_manager: &InputManager<IngameBindings>,
    movement_data: &MovementData,
    velocity: &mut Velocity,
    // decr_velocity: &mut DecreaseVelocity, // TODO
    base_friction_opt: &mut Option<&mut BaseFriction>,
) {
    let mut friction_enabled = true;
    let axis_binding = IngameAxisBinding::from(axis.clone());
    if let Some(val) = input_manager.axis_value(axis_binding) {
        let limit_max = |max_vel: f32| -> f32 { max_vel * val.abs() };

        if val != 0.0 {
            let (acceleration_opt, max_velocity_opt) = match &axis {
                Axis::X => (
                    movement_data.acceleration.0,
                    movement_data.max_velocity.0.map(limit_max),
                ),
                Axis::Y => (
                    movement_data.acceleration.1,
                    movement_data.max_velocity.1.map(limit_max),
                ),
            };

            if let Some(acceleration) = acceleration_opt {
                let speed = acceleration * val * dt;
                if let Some(max_velocity) = max_velocity_opt {
                    velocity.increase_with_max(&axis, speed, max_velocity);
                } else {
                    velocity.increase(&axis, speed);
                }

                friction_enabled =
                    velocity.get(&axis).signum() != speed.signum();

                // TODO
                // match speed {
                //     s if s > 0.0 => decr_velocity.dont_decrease_x_when_pos(),
                //     s if s < 0.0 => decr_velocity.dont_decrease_x_when_neg(),
                //     _ => (),
                // }
            }
        }
    }

    if let Some(base_friction) = base_friction_opt {
        base_friction.set_enabled(&axis, friction_enabled);
    }
}
