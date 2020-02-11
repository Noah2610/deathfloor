use super::component_prelude::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub enum MoveAction {
    WalkX(f32),
    WalkY(f32),
    Jump,
}

impl MoveAction {
    fn value(&self) -> i8 {
        let mut val = 0;
        match self {
            MoveAction::Jump => val += 0,
            MoveAction::WalkX(spd) => {
                val += 2;
                val += spd.signum() as i8;
            }
            MoveAction::WalkY(spd) => {
                val += 5;
                val += spd.signum() as i8;
            }
        }
        val
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

/// A `Movable` entity is moved by the `MovementSystem`.
#[derive(Default, Component)]
#[storage(DenseVecStorage)]
pub struct Movable {
    pub actions: HashSet<MoveAction>,
}

impl Movable {
    pub fn add_action(&mut self, action: MoveAction) {
        self.actions.insert(action);
    }
}
