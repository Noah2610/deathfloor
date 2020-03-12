use super::system_prelude::*;

#[derive(Default)]
struct QueryMatches {
    bottom: bool,
    left:   bool,
    right:  bool,
}

fn get_query_matches_from<'a>(
    collider: &'a Collider<CollisionTag>,
) -> QueryMatches {
    use deathframe::physics::query::exp::prelude_variants::*;

    let mut matches = QueryMatches::default();

    matches.bottom = collider
        .query::<FindQuery<CollisionTag>>()
        .exp(And(vec![
            IsTag(CollisionTag::Tile),
            Or(vec![IsState(Enter), IsState(Steady)]),
            IsSide(Bottom),
        ]))
        .run()
        .is_some();
    matches.left = collider
        .query::<FindQuery<CollisionTag>>()
        .exp(And(vec![
            IsTag(CollisionTag::Tile),
            Or(vec![IsState(Enter), IsState(Steady)]),
            IsSide(Left),
        ]))
        .run()
        .is_some();
    matches.right = collider
        .query::<FindQuery<CollisionTag>>()
        .exp(And(vec![
            IsTag(CollisionTag::Tile),
            Or(vec![IsState(Enter), IsState(Steady)]),
            IsSide(Right),
        ]))
        .run()
        .is_some();

    matches
}

#[derive(Default)]
pub struct ControlPlayerJumpSystem;

impl<'a> System<'a> for ControlPlayerJumpSystem {
    type SystemData = (
        Read<'a, InputManager<IngameBindings>>,
        WriteStorage<'a, Jumper>,
        ReadStorage<'a, WallJumper>,
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
            wall_jumpers,
            colliders,
            movement_data_store,
            mut movables,
            mut gravities,
        ): Self::SystemData,
    ) {
        for (
            jumper,
            wall_jumper_opt,
            collider,
            movement_data,
            movable,
            mut gravity_opt,
        ) in (
            &mut jumpers,
            wall_jumpers.maybe(),
            &colliders,
            &movement_data_store,
            &mut movables,
            (&mut gravities).maybe(),
        )
            .join()
        {
            let query_matches = get_query_matches_from(collider);
            let is_touching_horz = query_matches.left || query_matches.right;

            let is_jump_key_down = input_manager.is_down(PlayerJump);

            let mut jumped = false;
            let mut killed_jump = false;

            // JUMP
            if is_jump_key_down && query_matches.bottom {
                movable.add_action(MoveAction::Jump {
                    strength: movement_data.jump_strength,
                });
                jumper.is_jumping = true;
                jumped = true;
            }

            // WALL JUMP
            if let Some(_wall_jumper) = wall_jumper_opt {
                if !jumped && is_jump_key_down && is_touching_horz {
                    #[rustfmt::skip]
                    let x_mult = match (query_matches.left, query_matches.right) {
                        (true,  true)  => 0.0,            // touching both sides, so no x boost
                        (true,  false) => 1.0,            // touching left, so jump to the right
                        (false, true)  => -1.0,           // touching right, so jump to the left
                        (false, false) => unreachable!(), // `is_touching_horz` is `true`, so this is unreachable
                    };

                    movable.add_action(MoveAction::WallJump {
                        strength: (
                            movement_data.wall_jump_strength.0 * x_mult,
                            movement_data.wall_jump_strength.1,
                        ),
                    });
                    jumper.is_jumping = true;
                    jumped = true;
                }
            }

            if !jumped && is_touching_horz && !query_matches.bottom {
                // SLIDE on wall
                movable.add_action(MoveAction::WallSlide {
                    strength: movement_data.wall_slide_strength,
                });
            }

            // KILL JUMP
            if jumper.is_jumping && input_manager.is_up(PlayerJump) {
                movable.add_action(MoveAction::KillJump {
                    strength:     movement_data.jump_kill_strength,
                    min_velocity: movement_data.min_jump_velocity,
                });
                jumper.is_jumping = false;
                killed_jump = true;
            }

            // set appropriate GRAVITY
            if jumped {
                maybe_set_gravity(
                    &mut gravity_opt,
                    &movement_data.jump_gravity,
                );
            } else if killed_jump {
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
