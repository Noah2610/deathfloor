use super::entity_config::prelude::*;
use crate::components::prelude::Merge;
use crate::map_loader::map_data::Props;
use crate::map_loader::types::TileType;
use deathframe::amethyst::{Error, Result};
use std::collections::HashMap;
use std::convert::TryFrom;

pub mod prelude {
    pub use super::TileSettings;
    pub use super::TilesSettings;
}

#[derive(Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct TilesSettings {
    pub types: HashMap<TileType, TileSettings>,
}

impl Merge for TilesSettings {
    fn merge(&mut self, other: Self) {
        let types = &mut self.types;
        for (other_type, other_settings) in other.types {
            if let Some(settings) = types.get_mut(&other_type) {
                settings.merge(other_settings);
            } else {
                types.insert(other_type, other_settings);
            }
        }
    }
}

#[derive(Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TileSettings {
    pub entity: Option<EntityConfig>,
}

impl Merge for TileSettings {
    fn merge(&mut self, other: Self) {
        *self = Self {
            entity: self.entity.take().merged(other.entity),
        };
    }
}

impl<'a> TryFrom<&'a Props> for TileSettings {
    type Error = Error;
    fn try_from(props: &'a Props) -> Result<Self> {
        let props_json = serde_json::ser::to_string(props)?;
        serde_json::de::from_str(&props_json).map_err(Into::into)
    }
}
