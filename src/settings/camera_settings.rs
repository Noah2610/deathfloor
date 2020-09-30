// resources/settings/camera.ron

use super::SizeSettings;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CameraSettings {
    pub size: SizeSettings,
    pub z:    f32,
}
