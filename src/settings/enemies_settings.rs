pub mod prelude {
    pub use super::EnemiesSettings;
    pub use super::EnemySettings;
}

use super::entity_config::prelude::*;
use crate::components::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Deserialize, Default)]
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
pub struct EnemySettings {
    /// Filename of the spritesheet to load for this enemy.
    pub spritesheet_filename: String,
    pub entity:               EntityConfig,
}
