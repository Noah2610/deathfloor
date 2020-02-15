use super::system_prelude::*;
use deathframe::physics::collision::prelude::*;

#[derive(Default)]
pub struct ControlPlayerJumpSystem;

impl<'a> System<'a> for ControlPlayerJumpSystem {
    type SystemData = (
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Collider<CollisionTag>>,
        ReadStorage<'a, MovementData>,
        WriteStorage<'a, Movable>,
    );

    fn run(
        &mut self,
        (
            input_manager,
            players,
            colliders,
            movement_data_store,
            mut movables,
        ): Self::SystemData,
    ) {
        for (_, collider, movement_data, movable) in
            (&players, &colliders, &movement_data_store, &mut movables).join()
        {
            let standing_on_solid =
                collider.collisions.values().any(|collision| {
                    if let CollisionState::Enter(state_data)
                    | CollisionState::Steady(state_data) = &collision.state
                    {
                        if let (CollisionSide::Bottom, CollisionTag::Tile) =
                            (&state_data.side, &state_data.tag)
                        {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });

            if standing_on_solid && input_manager.is_down(PlayerJump) {
                let jump_strength = movement_data.jump_strength;
                movable.add_action(MoveAction::Jump(jump_strength));
            }
        }
    }
}
