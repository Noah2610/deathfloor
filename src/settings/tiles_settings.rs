use super::hitbox_config::HitboxConfig;
use crate::components::prelude::Jumppad;
use crate::map_loader::types::TileType;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct TilesSettings {
    pub types: HashMap<TileType, TileSettings>,
}

#[derive(Clone, Deserialize)]
pub struct TileSettings {
    pub is_solid: bool,
    pub hitbox:   Option<HitboxConfig>,
    pub jumppad:  Option<Jumppad>,
}
