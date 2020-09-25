use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadExpect<'a, InputManager<IngameBindings>>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Facing>,
        ReadStorage<'a, Player>,
    );

    fn run(
        &mut self,
        (
            time,
            input_manager,
            mut movables,
            mut facing_store,
            player_store,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (movable, mut facing, _) in
            (&mut movables, (&mut facing_store).maybe(), &player_store).join()
        {
            Axis::for_each(|axis| {
                handle_move_on_axis(
                    axis,
                    dt,
                    &input_manager,
                    movable,
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
    movable: &mut Movable,
    facing_opt: &mut Option<&mut Facing>,
) {
    let axis_binding = IngameAxisBinding::from(axis.clone());
    if let Some(val) = input_manager.axis_value(axis_binding) {
        if val != 0.0 {
            // TODO
            // This used to set the player's max movement velocity,
            // so if they where playing with controller and just slightly
            // pushed the joystick, they would only move slightly.
            // This doesn't quite work anymore without physics data.
            //
            // let limit_max = |max_vel: f32| -> f32 { max_vel * val.abs() };
            // max_movement_velocity_opt.as_mut().map(|maxvel| {
            //     maxvel.set_opt(
            //         &axis,
            //         physics_data.max_velocity.by_axis(&axis).map(limit_max),
            //     )
            // });

            if let Axis::X = &axis {
                if let Some(facing) = facing_opt {
                    **facing = Facing::from(val);
                }
            }

            movable.add_action(MoveAction::Walk {
                axis,
                mult: val * dt,
            });
        }
    }
}
