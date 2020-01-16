use super::SizeSettings;

#[derive(Clone, Deserialize)]
pub struct CameraSettings {
    pub size: SizeSettings,
    pub z:    f32,
}
