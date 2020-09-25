use super::component_prelude::*;

/// When player is standing on ground and stops moving,
/// their velocity is killed.
/// This component specifies the minimum amount of velocity
/// the player should still have after they stopped moving.
/// If set to `0.0`, they will instantly stop moving.
#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct KillVelocityMin {
    /// The minimum amount of velocity the player should
    /// when killing their velocity after they stop moving on the ground.
    /// Should be positive.
    pub min_velocity: f32,
}
