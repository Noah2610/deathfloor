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

impl ActionQueue for Movable {
    type Action = MoveAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
