use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, PhysicsData>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, MaxMovementVelocity>,
        WriteStorage<'a, Facing>,
    );

    fn run(
        &mut self,
        (
            time,
            input_manager,
            physics_data_store,
            mut movables,
            mut max_movement_velocities,
            mut facing_store,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (physics_data, movable, mut max_velocity_opt, mut facing) in (
            &physics_data_store,
            &mut movables,
            (&mut max_movement_velocities).maybe(),
            (&mut facing_store).maybe(),
        )
            .join()
        {
            Axis::for_each(|axis| {
                handle_move_on_axis(
                    axis,
                    dt,
                    &input_manager,
                    physics_data,
                    movable,
                    &mut max_velocity_opt,
                    &mut facing,
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
    movable: &mut Movable,
    max_movement_velocity_opt: &mut Option<&mut MaxMovementVelocity>,
    facing_opt: &mut Option<&mut Facing>,
) {
    let axis_binding = IngameAxisBinding::from(axis.clone());
    if let Some(val) = input_manager.axis_value(axis_binding) {
        if val != 0.0 {
            // TODO
            // let limit_max = |max_vel: f32| -> f32 { max_vel * val.abs() };
            // max_movement_velocity_opt.as_mut().map(|maxvel| {
            //     maxvel.set_opt(
            //         &axis,
            //         physics_data.max_velocity.by_axis(&axis).map(limit_max),
            //     )
            // });

            if &axis == &Axis::X {
                if let Some(facing) = facing_opt {
                    **facing = Facing::from(val);
                }
            }

            let acceleration_opt = physics_data.acceleration.by_axis(&axis);

            if let Some(acceleration) = acceleration_opt {
                let speed = acceleration * val * dt;
                movable.add_action(MoveAction::Walk { axis, speed });
            }
        }
    }
}
