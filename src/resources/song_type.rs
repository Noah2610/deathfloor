use crate::helpers::resource;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Debug)]
pub enum SongType {
    Floor1,
    Cntrlgun,
}

impl SongType {
    pub fn path(&self) -> PathBuf {
        let sfx_dir = resource("audio/bgm");
        match self {
            SongType::Floor1 => sfx_dir.join("floor1wip.ogg"),
            SongType::Cntrlgun => sfx_dir.join("cntrlgun.ogg"),
        }
    }
}
