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

#[derive(Clone, Deserialize)]
pub struct TilesSettings {
    pub types: HashMap<TileType, TileSettings>,
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
    /// `self` takes precedence.
    /// Takes ownership of `self`, and returns a new `Self`.
    fn merge(self, other: Self) -> Self {
        Self {
            hitbox:        self.hitbox.or(other.hitbox),
            jumppad:       self.jumppad.or(other.jumppad),
            collision_tag: self.collision_tag.or(other.collision_tag),
            solid_tag:     self.solid_tag.or(other.solid_tag),

            is_solid: self.is_solid.or(other.is_solid),

            jumppad_strength_x: self
                .jumppad_strength_x
                .or(other.jumppad_strength_x),
            jumppad_strength_y: self
                .jumppad_strength_y
                .or(other.jumppad_strength_y),
        }
    }
}

impl<'a> TryFrom<&'a Props> for TileSettings {
    type Error = Error;
    fn try_from(props: &'a Props) -> Result<Self> {
        let props_json = serde_json::ser::to_string(props)?;
        serde_json::de::from_str(&props_json).map_err(Into::into)
    }
}
