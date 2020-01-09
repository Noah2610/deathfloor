use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, MovementData>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            time,
            input_manager,
            players,
            movement_data_store,
            mut velocities,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (_, player_movement_data, player_velocity) in
            (&players, &movement_data_store, &mut velocities).join()
        {
            if let Some(x) = input_manager.axis_value(PlayerX) {
                if x != 0.0 {
                    if let Some(acceleration) =
                        player_movement_data.acceleration.0
                    {
                        player_velocity.increase_x_with_max(
                            acceleration * x * dt,
                            player_movement_data
                                .max_velocity
                                .0
                                .map(|max| max * x.abs()),
                        );
                    }
                }
            }
            if let Some(y) = input_manager.axis_value(PlayerY) {
                if y != 0.0 {
                    if let Some(acceleration) =
                        player_movement_data.acceleration.1
                    {
                        player_velocity.increase_y_with_max(
                            acceleration * y * dt,
                            player_movement_data
                                .max_velocity
                                .1
                                .map(|max| max * y.abs()),
                        );
                    }
                }
            }
        }
    }
}
