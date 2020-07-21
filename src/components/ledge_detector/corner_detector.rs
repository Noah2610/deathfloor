use super::prelude::*;

#[derive(Clone, Deserialize)]
#[serde(from = "LedgeDetectorCornerDetectorData")]
pub struct LedgeDetectorCornerDetector {
    corner:      LedgeDetectorCorner,
    if_touching: LedgeDetectorSide,
}

// Used for deserialization / configuration
#[derive(Clone, Deserialize)]
pub struct LedgeDetectorCornerDetectorData {
    #[serde(flatten)]
    corner_detector: LedgeDetectorCornerDetector,
    #[serde(default)]
    offset:          (f32, f32),
    #[serde(default = "default_corner_detector_size")]
    size:            (f32, f32),
}

impl From<LedgeDetectorCornerDetectorData> for LedgeDetectorCornerDetector {
    fn from(data: LedgeDetectorCornerDetectorData) -> Self {
        data.corner_detector
    }
}

const fn default_corner_detector_size() -> (f32, f32) {
    (4.0, 4.0)
}
