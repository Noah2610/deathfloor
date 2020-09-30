use crate::entity_config::prelude::EntityConfigComponents;

/// Insert (add or overwrite existing) components.
/// Can be given the same components as
/// in the enemy config's `components` field.
#[derive(Clone, Deserialize)]
#[serde(from = "EntityConfigComponents")]
pub struct InsertComponents(pub EntityConfigComponents);

impl From<EntityConfigComponents> for InsertComponents {
    fn from(components: EntityConfigComponents) -> Self {
        Self(components)
    }
}
