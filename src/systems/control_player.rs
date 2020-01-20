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
    // let is_velocity_below_max = |rigid: &RigidBody| {
    //     let (velocity, max_velocity_opt) = {
    //         let vel = rigid.velocity().as_vector();
    //         match &axis {
    //             Axis::X => (vel.x, physics_data.max_velocity.0),
    //             Axis::Y => (vel.y, physics_data.max_velocity.1),
    //         }
    //     };
    //     max_velocity_opt
    //         .map(|max_vel| velocity.abs() < max_vel)
    //         .unwrap_or(true)
    // };
    // let is_velocity_above_max =
    //     |rigid: &RigidBody| !is_velocity_below_max(rigid);

    let axis_binding = IngameAxisBinding::from(&axis);
    // let center_of_mass = rigid_body.center_of_mass();
    if let Some(val) = input_manager.axis_value(axis_binding) {
        if val != 0.0 {
            if let Some(acceleration) = physics_data.acceleration.0 {
                let speed = acceleration * val; // * dt; // NOTE: dt is included with some ForceTypes / physics bundle

                // let should_apply_force = is_velocity_below_max(rigid_body);

                // Apply force if below max velocity
                // if should_apply_force {
                let force = Force::linear({
                    (match &axis {
                        Axis::X => Vector::x(),
                        Axis::Y => Vector::y(),
                    }) * dbg!(speed)
                });

                rigid_body.apply_force(
                    0,
                    &force,
                    ForceType::AccelerationChange,
                    true,
                );
                // }

                // Set to max velocity, if force was applied and is now above max velocity
                // if should_apply_force && is_velocity_above_max(rigid_body) {
                //     let velocity = {
                //         let vel = rigid_body.velocity().as_vector();
                //         (vel.x, vel.y)
                //     };
                //     match &axis {
                //         Axis::X => {
                //             if let Some(max_x) = physics_data.max_velocity.0 {
                //                 if velocity.0 > max_x {
                //                     rigid_body.set_velocity(Velocity::linear(
                //                         max_x, velocity.1,
                //                     ));
                //                     rigid_body.clear_forces();
                //                 }
                //             }
                //         }

                //         Axis::Y => {
                //             if let Some(max_y) = physics_data.max_velocity.1 {
                //                 if velocity.1 > max_y {
                //                     rigid_body.set_velocity(Velocity::linear(
                //                         velocity.0, max_y,
                //                     ));
                //                     rigid_body.clear_forces();
                //                 }
                //             }
                //         }
                //     }
                // }

                dbg!(rigid_body.velocity());
            }
        }
    }
}
