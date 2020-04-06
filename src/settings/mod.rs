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
use crate::merge::Merge;
use deathframe::amethyst;
use prelude::*;
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
            tiles:   Self::load_dir::<TilesSettings, _>("tiles")?,
            enemies: Self::load_dir::<EnemiesSettings, _>("enemies")?,
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

    fn load_dir<T, S>(dirname: S) -> amethyst::Result<T>
    where
        for<'de> T: serde::Deserialize<'de> + Merge + Default,
        S: std::fmt::Display,
    {
        let path = resource(format!("settings/{}", dirname));
        let errmsg = format!("No settings files found in {:?}", &path);
        let all_settings = Self::load_configs_recursively_from(path)?;
        let merged_settings = Self::merge_settings(all_settings)
            .unwrap_or_else(|| {
                eprintln!(
                    "[WARNING]\n    {}\n    Using default (probably empty \
                     settings)",
                    errmsg
                );
                T::default()
            });
        Ok(merged_settings)
    }

    fn load_configs_recursively_from<T>(
        path: PathBuf,
    ) -> amethyst::Result<Vec<T>>
    where
        for<'de> T: serde::Deserialize<'de> + Merge,
    {
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
                            "Failed parsing RON settings file: {:?}\n{:#?}",
                            entry_path, e
                        ))
                    })?);
                }
            } else if entry_path.is_dir() {
                settings.append(&mut Self::load_configs_recursively_from(
                    entry_path,
                )?);
            }
        }

        Ok(settings)
    }

    /// Merge `Vec` of settings `T` together.
    /// Returns `None` if given `Vec` is empty.
    fn merge_settings<T>(all_settings: Vec<T>) -> Option<T>
    where
        T: Merge,
    {
        let mut merged_settings: Option<T> = None;
        for settings in all_settings {
            if let Some(merged) = merged_settings.as_mut() {
                merged.merge(settings);
            } else {
                merged_settings = Some(settings);
            }
        }
        merged_settings
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
