use super::geo::{LedgeDetectorCorner, LedgeDetectorSide};

pub enum LedgeDetectorAction {
    Detected(LedgeDetectorCorner, LedgeDetectorSide),
}
