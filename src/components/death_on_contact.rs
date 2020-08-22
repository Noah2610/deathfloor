use super::component_prelude::*;

/// This entity dies when it collides with the given collision tag.
/// Sets the entity's `Lifecycle` state to `Death`.
#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct DeathOnContact {
    pub collides_with: Vec<CollisionLabel>,
}
