use super::system_prelude::*;

// TODO

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, MovementData>,
        // WriteStorage<'a, Velocity>,
        // WriteStorage<'a, DecreaseVelocity>,
    );

    fn run(
        &mut self,
        (
            time,
            input_manager,
            players,
            movement_data_store,
            // mut velocities,
            // mut decr_velocities,
        ): Self::SystemData,
    ) {
        // let dt = time.delta_seconds() as f32;

        // for (_, player_movement_data, player_velocity, player_decr_velocity) in
        //     (
        //         &players,
        //         &movement_data_store,
        //         &mut velocities,
        //         &mut decr_velocities,
        //     )
        //         .join()
        // {
        //     Axis::for_each(|axis| {
        //         handle_move_on_axis(
        //             axis,
        //             dt,
        //             &input_manager,
        //             player_movement_data,
        //             player_velocity,
        //             player_decr_velocity,
        //         );
        //     });
        // }
    }
}

/*
fn handle_move_on_axis(
    axis: Axis,
    dt: f32,
    input_manager: &InputManager<IngameBindings>,
    movement_data: &MovementData,
    velocity: &mut Velocity,
    decr_velocity: &mut DecreaseVelocity,
) {
    let axis_binding = IngameAxisBinding::from(axis);
    if let Some(val) = input_manager.axis_value(axis_binding) {
        if val != 0.0 {
            if let Some(acceleration) = movement_data.acceleration.0 {
                let speed = acceleration * val * dt;
                velocity.increase_x_with_max(
                    speed,
                    movement_data.max_velocity.0.map(|max| max * val.abs()),
                );
                match speed {
                    s if s > 0.0 => decr_velocity.dont_decrease_x_when_pos(),
                    s if s < 0.0 => decr_velocity.dont_decrease_x_when_neg(),
                    _ => (),
                }
            }
        }
    }
}
*/
