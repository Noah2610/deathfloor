#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum TileType {
    #[serde(rename = "")]
    None,
    Ground,
    Jumppad,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::None
    }
}
