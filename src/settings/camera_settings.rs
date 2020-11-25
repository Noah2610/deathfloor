// resources/settings/camera.ron

use super::SizeSettings;
use deathframe::amethyst::utils::ortho_camera::CameraOrthoWorldCoordinates;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CameraSettings {
    pub size:              SizeSettings,
    pub z:                 f32,
    pub world_coordinates: CameraOrthoWorldCoordinates,
}
