pub mod prelude {
    pub use super::camera_settings::CameraSettings;
    pub use super::physics_data::PhysicsData;
    pub use super::player_settings::PlayerSettings;
    pub use super::tiles_settings::{TileSettings, TileType, TilesSettings};
    pub use super::Settings;
    pub use super::SizeSettings;
}

pub mod camera_settings;
pub mod physics_data;
pub mod player_settings;
pub mod tiles_settings;

use deathframe::amethyst;
use prelude::*;
use serde::Deserialize;

#[derive(Clone)]
pub struct Settings {
    pub camera: CameraSettings,
    pub player: PlayerSettings,
    pub tiles:  TilesSettings,
}

impl Settings {
    pub fn load() -> amethyst::Result<Self> {
        Ok(Settings {
            camera: Self::load_file::<CameraSettings, _>("camera.ron")?,
            player: Self::load_file::<PlayerSettings, _>("player.ron")?,
            tiles:  Self::load_file::<TilesSettings, _>("tiles.ron")?,
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
