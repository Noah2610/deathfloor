use deathframe::amethyst;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum TileType {
    #[serde(rename = "")]
    None,
    Ground,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::None
    }
}

impl TryFrom<&str> for TileType {
    type Error = amethyst::Error;
    fn try_from(s: &str) -> amethyst::Result<Self> {
        // In JSON, this would be a string, so the input string needs to be wrapped in "".
        let s = format!("\"{}\"", s);
        serde_json::from_str::<Self>(&s).map_err(|_| {
            amethyst::Error::from_string(format!("Invalid tile type: {}", s))
        })
    }
}

impl TryFrom<String> for TileType {
    type Error = amethyst::Error;
    fn try_from(s: String) -> amethyst::Result<Self> {
        Self::try_from(s.as_str())
    }
}
