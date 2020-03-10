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
