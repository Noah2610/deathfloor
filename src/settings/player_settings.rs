use super::SizeSettings;
use crate::components::prelude::PhysicsData;

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub size:    SizeSettings,
    pub physics: PhysicsData,
}
