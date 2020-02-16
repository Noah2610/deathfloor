use super::system_prelude::*;
use deathframe::physics::collision::prelude::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct ControlPlayerJumpSystem {
    players_jumping: HashSet<Entity>,
}

impl<'a> System<'a> for ControlPlayerJumpSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Collider<CollisionTag>>,
        ReadStorage<'a, MovementData>,
        WriteStorage<'a, Movable>,
        WriteStorage<'a, Gravity>,
    );

    fn run(
        &mut self,
        (
            entities,
            input_manager,
            players,
            colliders,
            movement_data_store,
            mut movables,
            mut gravities,
        ): Self::SystemData,
    ) {
        for (entity, _, collider, movement_data, movable, mut gravity_opt) in (
            &entities,
            &players,
            &colliders,
            &movement_data_store,
            &mut movables,
            (&mut gravities).maybe(),
        )
            .join()
        {
            let is_standing_on_solid =
                collider.collisions.values().any(|collision| {
                    if let CollisionState::Enter(state_data)
                    | CollisionState::Steady(state_data) = &collision.state
                    {
                        let collisions_match =
                            collider.tag.collides_with(&state_data.tag);
                        // if let (CollisionSide::Bottom, true) =
                        //     (&state_data.side, collisions_match)
                        if let (_, true) = (&state_data.side, collisions_match)
                        {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });

            if is_standing_on_solid && input_manager.is_down(PlayerJump) {
                movable.add_action(MoveAction::Jump {
                    strength: movement_data.jump_strength,
                });
                self.set_jumping(entity, true);
            }
            if self.is_jumping(&entity) && input_manager.is_up(PlayerJump) {
                movable.add_action(MoveAction::KillJump {
                    strength:     movement_data.jump_kill_strength,
                    min_velocity: movement_data.min_jump_velocity,
                });
                self.set_jumping(entity, false);
            }

            if self.is_jumping(&entity) {
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

impl ControlPlayerJumpSystem {
    fn set_jumping(&mut self, entity: Entity, jumping: bool) {
        if jumping {
            self.players_jumping.insert(entity);
        } else {
            self.players_jumping.remove(&entity);
        }
    }

    fn is_jumping(&self, entity: &Entity) -> bool {
        self.players_jumping.contains(&entity)
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
