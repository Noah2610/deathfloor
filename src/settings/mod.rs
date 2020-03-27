pub mod prelude {
    pub use super::camera_settings::CameraSettings;
    pub use super::enemies_settings::prelude::*;
    pub use super::general_settings::GeneralSettings;
    pub use super::hitbox_config::HitboxConfig;
    pub use super::player_settings::prelude::*;
    pub use super::tiles_settings::prelude::*;
    pub use super::Settings;
    pub use super::SizeSettings;
}

mod camera_settings;
mod enemies_settings;
mod general_settings;
mod hitbox_config;
mod player_settings;
mod tiles_settings;

use deathframe::amethyst;
use prelude::*;

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub general: GeneralSettings,
    pub camera:  CameraSettings,
    pub player:  PlayerSettings,
    pub tiles:   TilesSettings,
    pub enemies: EnemiesSettings,
}

impl Settings {
    pub fn load() -> amethyst::Result<Self> {
        Ok(Settings {
            general: Self::load_file::<GeneralSettings, _>("general.ron")?,
            camera:  Self::load_file::<CameraSettings, _>("camera.ron")?,
            player:  Self::load_file::<PlayerSettings, _>("player.ron")?,
            tiles:   Self::load_file::<TilesSettings, _>("tiles.ron")?,
            enemies: Self::load_file::<EnemiesSettings, _>("enemies.ron")?,
        })
    }

    fn load_file<T, S>(filename: S) -> amethyst::Result<T>
    where
        for<'de> T: serde::Deserialize<'de>,
        S: std::fmt::Display,
    {
        use crate::helpers::resource;
        use std::fs::File;

        let file = File::open(resource(format!("settings/{}", filename)))?;
        Ok(ron::de::from_reader(file).map_err(|e| {
            amethyst::Error::from_string(format!(
                "Failed parsing ron settings file: {}\n{:#?}",
                filename, e
            ))
        })?)
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
