use crate::helpers::resource;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Debug)]
pub enum SongType {
    Ingame,
    Menu,
}

impl SongType {
    pub fn path(&self) -> PathBuf {
        let sfx_dir = resource("audio/bgm");
        match self {
            SongType::Ingame => sfx_dir.join("9temp.ogg"),
            SongType::Menu => sfx_dir.join("paulsesh.ogg"),
        }
    }
}
