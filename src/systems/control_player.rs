use super::system_prelude::*;
use crate::components::player::PlayerData;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        WriteRigidBodies<'a>,
    );

    fn run(
        &mut self,
        (input_manager, players, mut rigid_bodies): Self::SystemData,
    ) {
        for (_, player_rigid_body) in (&players, &mut rigid_bodies).join() {
            Axis::for_each(|axis| {
                handle_move_on_axis(axis, &input_manager, player_rigid_body);
            });
        }
    }
}

fn handle_move_on_axis(
    axis: Axis,
    input_manager: &InputManager<IngameBindings>,
    rigid_body: &mut RigidBody,
) {
    let axis_binding = IngameAxisBinding::from(&axis);
    if let Some(val) = input_manager.axis_value(axis_binding) {
        if val != 0.0 {
            let acceleration_opt = {
                if let Some(user_data) = rigid_body
                    .user_data()
                    .and_then(|data| data.downcast_ref::<PlayerData>())
                {
                    match &axis {
                        Axis::X => user_data.acceleration.0,
                        Axis::Y => user_data.acceleration.1,
                    }
                } else {
                    None
                }
            };

            if let Some(acceleration) = acceleration_opt {
                let speed = acceleration * val; // * dt; // NOTE: dt is included with some ForceTypes / physics bundle

                let force = Force::linear({
                    (match &axis {
                        Axis::X => Vector::x(),
                        Axis::Y => Vector::y(),
                    }) * speed
                });

                rigid_body.apply_force(
                    0,
                    &force,
                    ForceType::AccelerationChange,
                    true,
                );
            }
        }
    }
}
