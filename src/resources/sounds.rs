use crate::helpers::resource;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::audio::{Mp3Format, Source, SourceHandle};
use deathframe::amethyst;
use std::collections::HashMap;
use std::path::PathBuf;

pub mod prelude {
    pub use super::SoundType;
    pub use super::Sounds;
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum SoundType {
    Jump,
    Shoot,
}

impl SoundType {
    fn path(&self) -> PathBuf {
        let sfx_dir = resource("audio/sfx");
        match self {
            SoundType::Jump => sfx_dir.join("jump.mp3"),
            SoundType::Shoot => sfx_dir.join("shoot.mp3"),
        }
    }
}

/// Sound effects manager.
pub struct Sounds {
    sounds: HashMap<SoundType, SourceHandle>,
}

impl Sounds {
    /// Load all sounds from sfx directory.
    pub fn load_sounds(
        &mut self,
        loader: &Loader,
        asset_storage: &AssetStorage<Source>,
    ) {
        self.load_sound(SoundType::Jump, loader, asset_storage);
        self.load_sound(SoundType::Shoot, loader, asset_storage);
    }

    fn load_sound(
        &mut self,
        sound_type: SoundType,
        loader: &Loader,
        asset_storage: &AssetStorage<Source>,
    ) {
        let filepath =
            sound_type.path().to_str().map(ToString::to_string).unwrap();
        self.sounds.insert(
            sound_type,
            loader.load(filepath, Mp3Format, (), asset_storage),
        );
    }
}

impl Default for Sounds {
    fn default() -> Self {
        Self {
            sounds: HashMap::new(),
        }
    }
}
