use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, MovementData>,
        // WriteStorage<'a, Velocity>, // TODO
        // WriteStorage<'a, DecreaseVelocity>, // TODO
    );

    fn run(
        &mut self,
        (
            time,
            input_manager,
            players,
            movement_data_store,
            // mut velocities, // TODO
            // mut decr_velocities, // TODO
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (
            _,
            player_movement_data,
            // player_velocity,      // TODO
            // player_decr_velocity, // TODO
        ) in (
            &players,
            &movement_data_store,
            // &mut velocities, // TODO
            // &mut decr_velocities, // TODO
        )
            .join()
        {
            Axis::for_each(|axis| {
                // TODO
                // handle_move_on_axis(
                //     axis,
                //     dt,
                //     &input_manager,
                //     player_movement_data,
                //     player_velocity,
                //     player_decr_velocity,
                // );
            });
        }
    }
}

// TODO
// fn handle_move_on_axis(
//     axis: Axis,
//     dt: f32,
//     input_manager: &InputManager<IngameBindings>,
//     movement_data: &MovementData,
//     velocity: &mut Velocity,
//     decr_velocity: &mut DecreaseVelocity,
// ) {
//     let axis_binding = IngameAxisBinding::from(axis);
//     if let Some(val) = input_manager.axis_value(axis_binding) {
//         if val != 0.0 {
//             if let Some(acceleration) = movement_data.acceleration.0 {
//                 let speed = acceleration * val * dt;
//                 velocity.increase_x_with_max(
//                     speed,
//                     movement_data.max_velocity.0.map(|max| max * val.abs()),
//                 );
//                 match speed {
//                     s if s > 0.0 => decr_velocity.dont_decrease_x_when_pos(),
//                     s if s < 0.0 => decr_velocity.dont_decrease_x_when_neg(),
//                     _ => (),
//                 }
//             }
//         }
//     }
// }
