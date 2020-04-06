use super::hitbox_config::HitboxConfig;
use crate::collision_tag::CollisionTagWrapper;
use crate::components::prelude::Jumppad;
use crate::map_loader::map_data::Props;
use crate::map_loader::types::TileType;
use crate::merge::Merge;
use deathframe::amethyst::{Error, Result};
use std::collections::HashMap;
use std::convert::TryFrom;

pub mod prelude {
    pub use super::TileSettings;
    pub use super::TilesSettings;
}

#[derive(Clone, Deserialize, Default)]
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
#[serde(default)]
pub struct TileSettings {
    pub hitbox:        Option<HitboxConfig>,
    pub jumppad:       Option<Jumppad>,
    pub collision_tag: Option<CollisionTagWrapper>,
    pub solid_tag:     Option<CollisionTagWrapper>,

    #[deprecated]
    pub is_solid: Option<bool>,

    // For tiled properties
    #[serde(alias = "jumppad_x")]
    pub jumppad_strength_x: Option<f32>,
    #[serde(alias = "jumppad_y")]
    pub jumppad_strength_y: Option<f32>,
}

impl Merge for TileSettings {
    /// Merges the field values from `other` into `self`.
    /// `other` takes precedence.
    /// Takes ownership of `self`, and returns a new `Self`.
    fn merge(&mut self, other: Self) {
        *self = Self {
            hitbox:        other.hitbox.or(self.hitbox.take()),
            jumppad:       other.jumppad.or(self.jumppad.take()),
            collision_tag: other.collision_tag.or(self.collision_tag.take()),
            solid_tag:     other.solid_tag.or(self.solid_tag.take()),

            is_solid: other.is_solid.or(self.is_solid.take()),

            jumppad_strength_x: other
                .jumppad_strength_x
                .or(self.jumppad_strength_x.take()),
            jumppad_strength_y: other
                .jumppad_strength_y
                .or(self.jumppad_strength_y.take()),
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
