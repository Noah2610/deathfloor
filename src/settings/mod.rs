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

use crate::helpers::resource;
use deathframe::amethyst;
use prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

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
            enemies: Self::load_enemies_config()?,
        })
    }

    fn load_file<T, S>(filename: S) -> amethyst::Result<T>
    where
        for<'de> T: serde::Deserialize<'de>,
        S: std::fmt::Display,
    {
        let file = File::open(resource(format!("settings/{}", filename)))?;
        Ok(ron::de::from_reader(file).map_err(|e| {
            amethyst::Error::from_string(format!(
                "Failed parsing ron settings file: {}\n{:#?}",
                filename, e
            ))
        })?)
    }

    fn load_enemies_config() -> amethyst::Result<EnemiesSettings> {
        let path = resource("settings/enemies");
        let all_enemies_settings = Self::load_enemies_configs_from(path)?;
        let enemies_settings =
            Self::merge_enemies_settings(all_enemies_settings);
        Ok(enemies_settings)
    }

    fn load_enemies_configs_from(
        path: PathBuf,
    ) -> amethyst::Result<Vec<EnemiesSettings>> {
        let mut settings = Vec::new();

        for entry in path.read_dir()? {
            let entry_path = entry?.path();
            if entry_path.is_file() {
                if let Some("ron") =
                    entry_path.extension().and_then(|e| e.to_str())
                {
                    let file = File::open(&entry_path)?;
                    settings.push(ron::de::from_reader(file).map_err(|e| {
                        amethyst::Error::from_string(format!(
                            "Failed parsing ron settings file: {:?}\n{:#?}",
                            entry_path, e
                        ))
                    })?);
                }
            } else if entry_path.is_dir() {
                settings
                    .append(&mut Self::load_enemies_configs_from(entry_path)?);
            }
        }

        Ok(settings)
    }

    fn merge_enemies_settings(
        all_enemies_settings: Vec<EnemiesSettings>,
    ) -> EnemiesSettings {
        let mut enemies_types = HashMap::new();
        for enemies_settings in all_enemies_settings {
            enemies_types.extend(enemies_settings.types);
        }
        EnemiesSettings {
            types: enemies_types,
        }
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
