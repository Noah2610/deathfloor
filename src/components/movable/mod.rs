pub mod prelude {
    pub use super::Movable;
    pub use super::MoveAction;
}

mod move_action;

pub use move_action::MoveAction;

use super::component_prelude::*;

/// A `Movable` entity is moved by the `MovementSystem`.
#[derive(Default, Component)]
#[storage(DenseVecStorage)]
pub struct Movable {
    actions: Vec<MoveAction>,
}

impl Movable {
    pub fn add_action(&mut self, action: MoveAction) {
        self.actions.push(action);
    }

    /// Iterates over all actions, removing them from the underlying `Vec`.
    /// Wrapper for `Vec::drain`.
    pub fn drain_actions(&mut self) -> std::vec::Drain<MoveAction> {
        self.actions.drain(..)
    }
}
