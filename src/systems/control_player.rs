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
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (
            _,
            player_movement_data,
            player_velocity,
            // player_decr_velocity, // TODO
        ) in (
            &players,
            &movement_data_store,
            &mut velocities,
            // &mut decr_velocities, // TODO
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
) {
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
                dbg!(&velocity);
                // TODO
                // match speed {
                //     s if s > 0.0 => decr_velocity.dont_decrease_x_when_pos(),
                //     s if s < 0.0 => decr_velocity.dont_decrease_x_when_neg(),
                //     _ => (),
                // }
            }
        }
    }
}
