use super::system_prelude::*;
use std::collections::HashMap;

// TODO: Move into settings
const NORMAL_GRAVITY_BELOW_Y_VEL: f32 = 0.0;

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
    let solid_tag = CollisionTag::from(CollisionLabel::solid());

    matches.bottom = collider
        .query::<FindQuery<CollisionTag>>()
        .exp(&And(vec![
            IsTag(solid_tag.clone()),
            Or(vec![IsState(Enter), IsState(Steady)]),
            IsSide(Bottom),
        ]))
        .run()
        .is_some();
    matches.left = collider
        .query::<FindQuery<CollisionTag>>()
        .exp(&And(vec![
            IsTag(solid_tag.clone()),
            Or(vec![IsState(Enter), IsState(Steady)]),
            IsSide(Left),
        ]))
        .run()
        .is_some();
    matches.right = collider
        .query::<FindQuery<CollisionTag>>()
        .exp(&And(vec![
            IsTag(solid_tag),
            Or(vec![IsState(Enter), IsState(Steady)]),
            IsSide(Right),
        ]))
        .run()
        .is_some();

    matches
}

#[derive(PartialEq, Debug)]
enum TargetGravity {
    Normal,
    Jump,
}

#[derive(Default)]
pub struct ControlPlayerJumpSystem {
    player_gravities: HashMap<Entity, TargetGravity>,
}

impl<'a> System<'a> for ControlPlayerJumpSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, InputManager<IngameBindings>>,
        WriteStorage<'a, Jumper>,
        ReadStorage<'a, WallJumper>,
        ReadStorage<'a, WallSlider>,
        ReadStorage<'a, Collider<CollisionTag>>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Gravity>,
        ReadStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            input_manager,
            mut jumpers,
            wall_jumpers,
            wall_sliders,
            colliders,
            mut movables,
            mut gravities,
            velocities,
        ): Self::SystemData,
    ) {
        for (
            entity,
            jumper,
            wall_jumper_opt,
            wall_slider_opt,
            collider,
            movable,
            gravity_opt,
            velocity,
        ) in (
            &entities,
            &mut jumpers,
            wall_jumpers.maybe(),
            wall_sliders.maybe(),
            &colliders,
            &mut movables,
            (&mut gravities).maybe(),
            &velocities,
        )
            .join()
        {
            // Set Jumper's original_gravity field
            if jumper.original_gravity.is_none() {
                if let Some(original_gravity) =
                    gravity_opt.as_ref().map(|g| (*g).clone())
                {
                    jumper.original_gravity = Some(original_gravity);
                }
            }

            let query_matches = get_query_matches_from(collider);
            let is_touching_horz = query_matches.left || query_matches.right;

            let is_jump_key_down = input_manager.is_down(PlayerJump);
            let is_jump_key_pressed = input_manager.is_pressed(PlayerJump);

            let mut jumped = false;

            // JUMP
            if is_jump_key_down && query_matches.bottom {
                movable.add_action(MoveAction::Jump);
                jumper.is_jumping = true;
                jumped = true;
            }

            // WALL JUMP
            if wall_jumper_opt.is_some() {
                if !jumped && is_jump_key_down && is_touching_horz {
                    #[rustfmt::skip]
                    let x_mult = match (query_matches.left, query_matches.right) {
                        (true,  true)  => 0.0,            // touching both sides, so no x boost
                        (true,  false) => 1.0,            // touching left, so jump to the right
                        (false, true)  => -1.0,           // touching right, so jump to the left
                        (false, false) => unreachable!(), // `is_touching_horz` is `true`, so this is unreachable
                    };
                    movable.add_action(MoveAction::WallJump { x_mult });
                    jumper.is_jumping = true;
                    jumped = true;
                }
            }

            // WALL SLIDE
            if let Some(wall_slider) = wall_slider_opt {
                if !jumped && is_touching_horz && !query_matches.bottom {
                    movable.add_action(MoveAction::WallSlide {
                        velocity: wall_slider.slide_velocity,
                    });
                }
            }

            // KILL JUMP
            if jumper.is_jumping && input_manager.is_up(PlayerJump) {
                movable.add_action(MoveAction::KillJump);
                jumper.is_jumping = false;
            }

            // set appropriate GRAVITY
            if let Some(mut gravity) = gravity_opt {
                let mut target_gravity_opt = None;

                let vel_y = velocity.get(&Axis::Y);
                if is_jump_key_pressed && vel_y >= NORMAL_GRAVITY_BELOW_Y_VEL {
                    target_gravity_opt = Some(TargetGravity::Jump);
                } else if !is_jump_key_pressed {
                    target_gravity_opt = Some(TargetGravity::Normal);
                } else if vel_y < NORMAL_GRAVITY_BELOW_Y_VEL {
                    target_gravity_opt = Some(TargetGravity::Normal);
                }

                if let Some(target_gravity) = target_gravity_opt {
                    if self
                        .player_gravities
                        .get(&entity)
                        .map(|prev_gravity| prev_gravity != &target_gravity)
                        .unwrap_or(true)
                    {
                        match &target_gravity {
                            TargetGravity::Normal => set_gravity(
                                &mut gravity,
                                jumper.original_gravity.as_ref().expect(
                                    "Jumper should have set original_gravity \
                                     by now",
                                ),
                            ),
                            TargetGravity::Jump => {
                                set_gravity(&mut gravity, &jumper.gravity)
                            }
                        }
                        self.player_gravities.insert(entity, target_gravity);
                    }
                }
            }
        }
    }
}

fn set_gravity(gravity: &mut Gravity, new_gravity: &Gravity) {
    gravity.x = new_gravity.x;
    gravity.y = new_gravity.y;
}
