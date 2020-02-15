use deathframe::core::geo::prelude::{Axis, ByAxis};
use std::hash::{Hash, Hasher};

pub enum MoveAction {
    Walk(Axis, f32),
    Jump(f32),
    KillJump(f32, f32),
}

impl MoveAction {
    fn value(&self) -> i8 {
        match self {
            MoveAction::Walk(axis, spd) => {
                spd.signum() as i8 + (2, 5).by_axis(&axis)
            }
            MoveAction::Jump(_) => 0,
            MoveAction::KillJump(_, _) => 7,
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
