use crate::settings::prelude::EntityComponentsData;

/// Insert (add or overwrite existing) components.
/// Can be given the same components as
/// in the enemy config's `components` field.
#[derive(Clone, Deserialize)]
#[serde(from = "EntityComponentsData")]
pub struct InsertComponents(pub EntityComponentsData);

impl From<EntityComponentsData> for InsertComponents {
    fn from(components: EntityComponentsData) -> Self {
        Self(components)
    }
}
