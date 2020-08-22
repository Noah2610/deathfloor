pub mod prelude {
    pub use super::CustomEntitiesSettings;
    pub use super::CustomEntitySettings;
}

use super::entity_config::prelude::*;
use crate::components::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct CustomEntitiesSettings {
    pub types: HashMap<String, CustomEntitySettings>,
}

impl Merge for CustomEntitiesSettings {
    fn merge(&mut self, other: Self) {
        let types = &mut self.types;
        types.extend(other.types);
    }
}

#[derive(Clone, Deserialize)]
pub struct CustomEntitySettings {
    #[serde(alias = "spritesheet")]
    pub spritesheet_filename: Option<String>,
    pub entity:               EntityConfig,
}
