pub mod prelude {
    pub use super::player_settings::PlayerSettings;
    pub use super::Settings;
    pub use super::SizeSettings;
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

#[derive(Clone, Deserialize)]
pub struct SizeSettings {
    pub width:  f32,
    pub height: f32,
}

use crate::components::prelude::Size as SizeComp;

impl Into<SizeComp> for SizeSettings {
    fn into(self) -> SizeComp {
        SizeComp::new(self.width, self.height)
    }
}
