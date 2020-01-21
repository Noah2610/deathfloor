use super::physics_data::PhysicsData;
use deathframe::amethyst;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Clone, Deserialize)]
pub struct TilesSettings {
    pub types: HashMap<TileType, TileSettings>,
}

#[derive(Clone, Deserialize)]
pub struct TileSettings {
    pub physics: PhysicsData,
}

#[derive(Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum TileType {
    Ground,
    None,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::None
    }
}

impl TileType {
    fn try_from_s<S>(s: S) -> amethyst::Result<Self>
    where
        S: Into<String>,
    {
        let s = s.into();
        match s.as_str() {
            "" => Ok(TileType::None),
            "Ground" => Ok(TileType::Ground),
            t => Err(amethyst::Error::from_string(format!(
                "Invalid tile type: {}",
                t
            ))),
        }
    }
}

impl TryFrom<&str> for TileType {
    type Error = amethyst::Error;
    fn try_from(s: &str) -> amethyst::Result<Self> {
        Self::try_from_s(s)
    }
}

impl TryFrom<String> for TileType {
    type Error = amethyst::Error;
    fn try_from(s: String) -> amethyst::Result<Self> {
        Self::try_from_s(s)
    }
}
