pub mod prelude {
    pub use super::player_settings::PlayerSettings;
    pub use super::Settings;
}

mod player_settings;

use deathframe::amethyst;

#[derive(Deserialize)]
pub struct Settings {}

impl Settings {
    pub fn load() -> amethyst::Result<Self> {
        use crate::helpers::resource;
        use std::fs::File;

        let file = File::open(resource("config/settings.ron"))?;

        Ok(ron::de::from_reader(file)?)
    }
}
