pub mod prelude {
    pub use super::AbstractEntitiesSettings;
    pub use super::AbstractEntitySettings;
    pub use super::AbstractEntityType;
}

use super::entity_config::prelude::*;
use crate::components::prelude::*;
use std::collections::HashMap;

pub type AbstractEntityType = String;

#[derive(Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct AbstractEntitiesSettings {
    pub types: HashMap<AbstractEntityType, AbstractEntitySettings>,
}

impl Merge for AbstractEntitiesSettings {
    fn merge(&mut self, other: Self) {
        let types = &mut self.types;
        types.extend(other.types);
    }
}

#[derive(Clone, Deserialize)]
#[serde(from = "EntityConfig")]
pub struct AbstractEntitySettings {
    pub entity: EntityConfig,
}

impl From<EntityConfig> for AbstractEntitySettings {
    fn from(entity: EntityConfig) -> Self {
        Self { entity }
    }
}
