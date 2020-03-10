use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerJumpSystem;

impl<'a> System<'a> for ControlPlayerJumpSystem {
    type SystemData = (
        Read<'a, InputManager<IngameBindings>>,
        WriteStorage<'a, Jumper>,
        ReadStorage<'a, Collider<CollisionTag>>,
        ReadStorage<'a, MovementData>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Gravity>,
    );

    fn run(
        &mut self,
        (
            input_manager,
            mut jumpers,
            colliders,
            movement_data_store,
            mut movables,
            mut gravities,
        ): Self::SystemData,
    ) {
        for (jumper, collider, movement_data, movable, mut gravity_opt) in (
            &mut jumpers,
            &colliders,
            &movement_data_store,
            &mut movables,
            (&mut gravities).maybe(),
        )
            .join()
        {
            let is_standing_on_solid = collider
                .query()
                .any({
                    use deathframe::physics::query::exp::prelude::*;
                    Or(vec![
                        And(vec![
                            IsTag(CollisionTag::Tile),
                            Or(vec![IsState(Enter), IsState(Steady)]),
                            IsSide(Bottom),
                        ]),
                        And(vec![
                            IsTag(CollisionTag::Player),
                            Or(vec![IsState(Enter), IsState(Steady)]),
                        ]),
                    ])
                })
                .run();

            if is_standing_on_solid && input_manager.is_down(PlayerJump) {
                movable.add_action(MoveAction::Jump {
                    strength: movement_data.jump_strength,
                });
                jumper.is_jumping = true;
            }
            if jumper.is_jumping && input_manager.is_up(PlayerJump) {
                movable.add_action(MoveAction::KillJump {
                    strength:     movement_data.jump_kill_strength,
                    min_velocity: movement_data.min_jump_velocity,
                });
                jumper.is_jumping = false;
            }

            if jumper.is_jumping {
                maybe_set_gravity(
                    &mut gravity_opt,
                    &movement_data.jump_gravity,
                );
            } else {
                maybe_set_gravity(&mut gravity_opt, &movement_data.gravity);
            }
        }
    }
}

fn maybe_set_gravity(
    gravity_opt: &mut Option<&mut Gravity>,
    gravity_strength: &(Option<f32>, Option<f32>),
) {
    if let Some(gravity_comp) = gravity_opt {
        for axis in Axis::iter() {
            if let Some(grav) = gravity_strength.by_axis(&axis) {
                gravity_comp.set(&axis, *grav);
            }
        }
    }
}
