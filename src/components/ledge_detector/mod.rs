pub mod prelude {
    pub use super::corner_detector::LedgeDetectorCornerDetector;
    pub use super::geo::LedgeDetectorCorner;
    pub use super::geo::LedgeDetectorSide;
    pub use super::LedgeDetector;
    pub use super::LedgeDetectorData;
    pub use super::LedgeDetectorDetected;
}

mod corner_detector;
mod geo;

use super::component_prelude::*;
use std::collections::hash_set::{self, HashSet};

#[derive(PartialEq, Eq, Hash)]
pub struct LedgeDetectorDetected {
    pub corner:      LedgeDetectorCorner,
    pub if_touching: LedgeDetectorSide,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct LedgeDetector {
    corner_entities: Vec<Entity>,
    detected:        HashSet<LedgeDetectorDetected>,
}

impl LedgeDetector {
    pub fn new(corner_entities: Vec<Entity>) -> Self {
        Self {
            corner_entities,
            detected: HashSet::default(),
        }
    }

    pub fn drain_corner_entities(&mut self) -> std::vec::Drain<Entity> {
        self.corner_entities.drain(..)
    }

    pub fn add_detected(&mut self, detected: LedgeDetectorDetected) {
        self.detected.insert(detected);
    }

    pub fn drain_detected(&mut self) -> hash_set::Drain<LedgeDetectorDetected> {
        self.detected.drain()
    }
}

#[derive(Clone, Deserialize)]
pub struct LedgeDetectorData {
    pub corners:       Vec<corner_detector::LedgeDetectorCornerDetectorData>,
    pub collides_with: Vec<CollisionLabel>,
}
