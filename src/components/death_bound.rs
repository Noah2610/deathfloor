use super::component_prelude::*;

/// A `DeathBound` entity will be deleted if the `DeathBound`'s
/// associated entity is deleted.
#[derive(Component)]
#[storage(VecStorage)]
pub struct DeathBound {
    pub entity: Entity,
}

impl DeathBound {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}
