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
    corner_entities: Vec<Entity>,
    actions:         Vec<LedgeDetectorAction>,
}

impl LedgeDetector {
    pub fn new(corner_entities: Vec<Entity>) -> Self {
        Self {
            corner_entities,
            actions: Default::default(),
        }
    }
}

impl ActionQueue for LedgeDetector {
    type Action = LedgeDetectorAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}

#[derive(Clone, Deserialize)]
pub struct LedgeDetectorData {
    pub corners:       Vec<corner_detector::LedgeDetectorCornerDetectorData>,
    pub collides_with: Vec<CollisionLabel>,
}
