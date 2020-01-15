use std::collections::HashMap;

mod propful;

pub mod prelude {
    pub use super::propful::Propful;
    pub use super::Level as LevelData;
    pub use super::Map as MapData;
    pub use super::Object as ObjectData;
    pub use super::Objects as ObjectsData;
    pub use super::Pos as PosData;
    pub use super::Props as PropsData;
    pub use super::Size as SizeData;
    pub use super::Tile as TileData;
    pub use super::Tiles as TilesData;
}

pub type Props = HashMap<String, serde_json::Value>;
pub type Tiles = Vec<Tile>;
pub type Objects = Vec<Object>;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Deserialize)]
pub struct Map {
    pub level:   Level,
    pub tiles:   Tiles,
    pub objects: Objects,
}

#[derive(Debug, Deserialize)]
pub struct Level {
    pub size:      Size,
    pub tile_size: Size,
}

#[derive(Debug, Deserialize)]
pub struct Tile {
    pub id:    usize,
    pub ts:    String,
    pub pos:   Pos,
    pub props: Props,
}

#[derive(Debug, Deserialize)]
pub struct Object {
    #[serde(rename = "type")]
    pub object_type: String,
    pub pos: Pos,
    pub size: Size,
    pub props: Props,
}
