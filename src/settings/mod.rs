pub mod prelude {
    pub use super::camera_settings::CameraSettings;
    pub use super::custom_entities_settings::CustomEntitiesSettings;
    pub use super::enemies_settings::prelude::*;
    pub use super::entity_config::prelude::*;
    pub use super::general_settings::GeneralSettings;
    pub use super::hitbox_config::HitboxConfig;
    pub use super::level_settings::LevelSettings;
    pub use super::player_bullet_settings::prelude::*;
    pub use super::player_settings::prelude::*;
    pub use super::tiles_settings::prelude::*;
    pub use super::Settings;
    pub use super::SizeSettings;
}

mod camera_settings;
mod custom_entities_settings;
mod enemies_settings;
mod entity_config;
mod general_settings;
mod hitbox_config;
mod level_settings;
mod player_bullet_settings;
mod player_settings;
mod tiles_settings;

use crate::components::prelude::Merge;
use crate::helpers::resource;
use deathframe::amethyst;
use prelude::*;
use std::fs::File;
use std::path::PathBuf;

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub general:         GeneralSettings,
    pub camera:          CameraSettings,
    pub level:           LevelSettings,
    pub player:          PlayerSettings,
    pub player_bullet:   PlayerBulletSettings,
    pub tiles:           TilesSettings,
    pub enemies:         EnemiesSettings,
    pub custom_entities: CustomEntitiesSettings,
}

impl Settings {
    pub fn load() -> amethyst::Result<Self> {
        Ok(Settings {
            general:         Self::load_file("general.ron")?,
            camera:          Self::load_file("camera.ron")?,
            level:           Self::load_file("levels.ron")?,
            player:          Self::load_file("player.ron")?,
            player_bullet:   Self::load_file("player_bullet.ron")?,
            tiles:           Self::load_dir("tiles")?,
            enemies:         Self::load_dir("enemies")?,
            custom_entities: Self::load_dir("custom_entities")?,
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
