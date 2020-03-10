use super::hitbox_config::HitboxConfig;
use crate::map_loader::types::TileType;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct TilesSettings {
    pub types: HashMap<TileType, TileSettings>,
}

#[derive(Clone, Deserialize)]
pub struct TileSettings {
    pub hitbox: Option<HitboxConfig>,
}
