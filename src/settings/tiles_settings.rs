use super::hitbox_config::HitboxConfig;
use crate::components::prelude::Jumppad;
use crate::map_loader::map_data::Props;
use crate::map_loader::types::TileType;
use deathframe::amethyst::{Error, Result};
use std::collections::HashMap;
use std::convert::TryFrom;

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

impl<'a> TryFrom<&'a Props> for TileSettings {
    type Error = Error;
    fn try_from(props: &'a Props) -> Result<Self> {
        let props_json = serde_json::ser::to_string(props)?;
        serde_json::de::from_str(&props_json).map_err(Into::into)
    }
}
