use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, PhysicsData>,
        WriteRigidBodies<'a>,
        // WriteStorage<'a, Velocity>,
        // WriteStorage<'a, DecreaseVelocity>,
    );

    fn run(
        &mut self,
        (
            time,
            input_manager,
            players,
            physics_data_store,
            mut rigid_bodies,
            // mut velocities,
            // mut decr_velocities,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (_, player_physics_data, player_rigid_body) in
            (&players, &physics_data_store, &mut rigid_bodies).join()
        {
            Axis::for_each(|axis| {
                handle_move_on_axis(
                    axis,
                    dt,
                    &input_manager,
                    player_physics_data,
                    player_rigid_body,
                );
            });
        }
    }
}

fn handle_move_on_axis(
    axis: Axis,
    dt: f32,
    input_manager: &InputManager<IngameBindings>,
    physics_data: &PhysicsData,
    rigid_body: &mut RigidBody,
    // velocity: &mut Velocity,
    // decr_velocity: &mut DecreaseVelocity,
) {
    let axis_binding = IngameAxisBinding::from(&axis);
    // let center_of_mass = rigid_body.center_of_mass();
    if let Some(val) = input_manager.axis_value(axis_binding) {
        if val != 0.0 {
            if let Some(acceleration) = physics_data.acceleration.0 {
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
