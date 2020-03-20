use super::hitbox_config::HitboxConfig;
use crate::components::prelude::Jumppad;
use crate::map_loader::map_data::Props;
use crate::map_loader::types::TileType;
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
    pub is_solid: bool,
    pub hitbox:   Option<HitboxConfig>,
    pub jumppad:  Option<Jumppad>,

    // For tiled properties
    #[serde(alias = "jumppad_x")]
    pub jumppad_strength_x: Option<f32>,
    #[serde(alias = "jumppad_y")]
    pub jumppad_strength_y: Option<f32>,
}

impl TileSettings {
    // TODO: This could be extracted into a trait
    /// Merges the field values from `other` into `self`.
    /// `self` takes precedence.
    /// Takes ownership of `self`, and returns a new `Self`.
    pub fn merge(mut self, other: Self) -> Self {
        self.is_solid = self.is_solid; // Just here for completeness sake.
        self.hitbox = self.hitbox.or(other.hitbox);
        self.jumppad = self.jumppad.or(other.jumppad);
        self.jumppad_strength_x =
            self.jumppad_strength_x.or(other.jumppad_strength_x);
        self.jumppad_strength_y =
            self.jumppad_strength_y.or(other.jumppad_strength_y);

        self
    }
}

impl<'a> TryFrom<&'a Props> for TileSettings {
    type Error = Error;
    fn try_from(props: &'a Props) -> Result<Self> {
        let props_json = serde_json::ser::to_string(props)?;
        serde_json::de::from_str(&props_json).map_err(Into::into)
    }
}
