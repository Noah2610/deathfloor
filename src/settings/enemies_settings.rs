pub mod prelude {
    pub use super::EnemiesSettings;
    pub use super::EnemyComponentsData;
    pub use super::EnemySettings;
}

use crate::components::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct EnemiesSettings {
    pub types: HashMap<EnemyType, EnemySettings>,
}

#[derive(Clone, Deserialize)]
pub struct EnemySettings {
    pub spritesheet_filename: String, // TODO
    pub components:           EnemyComponentsData,
}

/// List of posible components for an Enemy.
/// All can optionally be added to an enemy's config.
#[derive(Clone, Deserialize)]
pub struct EnemyComponentsData {
    pub gravity: Option<Gravity>,
}
