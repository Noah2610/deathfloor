use super::component_prelude::*;

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Player;

/// Used as the the player's RigidBody's user_data.
#[derive(Debug, Clone)]
pub struct PlayerData {
    pub acceleration:  (Option<f32>, Option<f32>),
    pub jump_strength: f32,
}
