use crate::helpers::resource;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Debug)]
pub enum SoundType {
    Jump,
    Shoot,
}

impl SoundType {
    pub fn path(&self) -> PathBuf {
        let sfx_dir = resource("audio/sfx");
        match self {
            SoundType::Jump => sfx_dir.join("jump.ogg"),
            SoundType::Shoot => sfx_dir.join("shoot.ogg"),
        }
    }
}
