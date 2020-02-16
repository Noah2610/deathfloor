use super::hitbox_config::HitboxConfig;
use deathframe::amethyst;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Clone, Deserialize)]
pub struct TilesSettings {
    pub types: HashMap<TileType, TileSettings>,
}

#[derive(Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum TileType {
    Ground,
    None,
    // TODO
    SGround,
    WeirdGround,
    Weird,
    Passable,
}

#[derive(Clone, Deserialize)]
pub struct TileSettings {
    pub hitbox: Option<HitboxConfig>,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::None
    }
}

impl TryFrom<&str> for TileType {
    type Error = amethyst::Error;
    fn try_from(s: &str) -> amethyst::Result<Self> {
        if s.is_empty() {
            Ok(TileType::default())
        } else {
            Ok(ron::de::from_str(s).map_err(|e| {
                amethyst::Error::from_string(format!(
                    "Invalid tile type: {}\n{}",
                    s, e
                ))
            })?)
        }
    }
}
