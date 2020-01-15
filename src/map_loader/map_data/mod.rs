use std::collections::HashMap;

type Props = HashMap<String, serde_json::Value>;

#[derive(Debug, Deserialize)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Deserialize)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Deserialize)]
pub struct Map {
    pub level:   Level,
    pub tiles:   Vec<Tile>,
    pub objects: Vec<Object>,
}

#[derive(Debug, Deserialize)]
pub struct Level {
    pub size:      Size,
    pub tile_size: Size,
}

#[derive(Debug, Deserialize)]
pub struct Tile {
    pub id:    f32,
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
