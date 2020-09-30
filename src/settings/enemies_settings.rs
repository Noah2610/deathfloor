// resources/entities/enemies

pub mod prelude {
    pub use super::EnemiesSettings;
    pub use super::EnemySettings;
}

use crate::components::prelude::*;
use crate::entity_config::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EnemiesSettings {
    pub types: HashMap<EnemyType, EnemySettings>,
}

impl Merge for EnemiesSettings {
    fn merge(&mut self, other: Self) {
        let types = &mut self.types;
        types.extend(other.types);
    }
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnemySettings {
    /// Filename of the spritesheet to load for this enemy.
    #[serde(alias = "spritesheet")]
    pub spritesheet_filename: String,
    pub entity:               EntityConfig,
}
