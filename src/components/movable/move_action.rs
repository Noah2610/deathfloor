use deathframe::core::geo::prelude::{Axis, ByAxis};
use std::hash::{Hash, Hasher};

pub enum MoveAction {
    Walk {
        axis:  Axis,
        speed: f32,
    },
    Jump {
        strength: f32,
        gravity:  (Option<f32>, Option<f32>),
    },
    KillJump {
        strength:     f32,
        gravity:      (Option<f32>, Option<f32>),
        min_velocity: f32,
    },
}

impl MoveAction {
    fn value(&self) -> i8 {
        match self {
            MoveAction::Walk { axis, speed } => {
                speed.signum() as i8 + (2, 5).by_axis(&axis)
            }
            MoveAction::Jump { .. } => 0,
            MoveAction::KillJump { .. } => 7,
        }
    }
}

impl PartialEq for MoveAction {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for MoveAction {
}

impl Hash for MoveAction {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.value().hash(state);
    }
}
