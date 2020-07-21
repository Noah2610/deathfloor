pub mod prelude {
    pub use super::corner_detector::LedgeDetectorCornerDetector;
    pub use super::geo::LedgeDetectorCorner;
    pub use super::geo::LedgeDetectorSide;
    pub use super::ledge_detector_action::LedgeDetectorAction;
    pub use super::LedgeDetector;
    pub use super::LedgeDetectorData;
}

mod corner_detector;
mod geo;
mod ledge_detector_action;

use super::component_prelude::*;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct LedgeDetector {
    actions: Vec<LedgeDetectorAction>,
}

impl ActionQueue for LedgeDetector {
    type Action = LedgeDetectorAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}

#[derive(Clone, Deserialize)]
pub struct LedgeDetectorData {
    corners: Vec<corner_detector::LedgeDetectorCornerDetectorData>,
}
