pub mod prelude {
    pub use super::Movable;
    pub use super::MoveAction;
}

mod move_action;

pub use move_action::MoveAction;

use super::component_prelude::*;
use std::collections::{hash_set, HashSet};

/// A `Movable` entity is moved by the `MovementSystem`.
#[derive(Default, Component)]
#[storage(DenseVecStorage)]
pub struct Movable {
    actions: HashSet<MoveAction>,
}

impl Movable {
    pub fn add_action(&mut self, action: MoveAction) {
        self.actions.insert(action);
    }

    /// Iterates over all actions, removing them from the underlying `HashSet`.
    /// Wrapper for `HashSet::drain`.
    pub fn drain_actions(&mut self) -> hash_set::Drain<MoveAction> {
        self.actions.drain()
    }
}
