use super::physics_data::PhysicsData;
use super::SizeSettings;

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub size:    SizeSettings,
    pub physics: PhysicsData,
}
