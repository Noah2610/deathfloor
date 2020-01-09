pub mod prelude {
    pub use super::player_settings::PlayerSettings;
    pub use super::Settings;
}

mod player_settings;

use deathframe::amethyst;
use prelude::*;

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub player: PlayerSettings,
}

impl Settings {
    pub fn load() -> amethyst::Result<Self> {
        use crate::helpers::resource;
        use std::fs::File;

        let file = File::open(resource("config/settings.ron"))?;

        Ok(ron::de::from_reader(file)?)
    }
}
