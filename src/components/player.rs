use super::component_prelude::*;

/// Used as the the player's RigidBody's user_data.
#[derive(Debug, Clone)]
pub struct PlayerData {
    pub acceleration: (Option<f32>, Option<f32>),
}

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Player;
