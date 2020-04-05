use super::sound_type::SoundType;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::audio::{Mp3Format, Source, SourceHandle};
use deathframe::amethyst;
use std::collections::HashMap;

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
