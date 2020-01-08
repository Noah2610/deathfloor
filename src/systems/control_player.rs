use super::system_prelude::*;

const PLAYER_ACCELERATION: f32 = 100.0;
const PLAYER_MAX_VELOCITY: f32 = 100.0;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (time, input_manager, players, mut velocities): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (_, player_velocity) in (&players, &mut velocities).join() {
            if let Some(x) = input_manager.axis_value(PlayerX) {
                dbg!(x);
                // if x != 0.0 {
                player_velocity.increase_x_with_max(
                    PLAYER_ACCELERATION * x * dt,
                    Some(PLAYER_MAX_VELOCITY),
                );
                // }
            }
            if let Some(y) = input_manager.axis_value(PlayerY) {
                dbg!(y);
                // if y != 0.0 {
                player_velocity.increase_y_with_max(
                    PLAYER_ACCELERATION * y * dt,
                    Some(PLAYER_MAX_VELOCITY),
                );
                // }
            }
        }
    }
}
