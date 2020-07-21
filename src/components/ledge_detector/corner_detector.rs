use super::*;

#[derive(Component, Clone, Builder)]
#[storage(VecStorage)]
#[builder(pattern = "owned")]
pub struct LedgeDetectorCornerDetector {
    corner:      LedgeDetectorCorner,
    if_touching: LedgeDetectorSide,
    owner:       Entity,
}

impl LedgeDetectorCornerDetector {
    pub fn builder() -> LedgeDetectorCornerDetectorBuilder {
        LedgeDetectorCornerDetectorBuilder::default()
    }
}

// Used for deserialization / configuration
#[derive(Clone, Deserialize)]
pub struct LedgeDetectorCornerDetectorData {
    pub corner:      LedgeDetectorCorner,
    pub if_touching: LedgeDetectorSide,
    #[serde(default)]
    pub offset:      (f32, f32),
    #[serde(default = "default_corner_detector_size")]
    pub size:        (f32, f32),
}

const fn default_corner_detector_size() -> (f32, f32) {
    (4.0, 4.0)
}
