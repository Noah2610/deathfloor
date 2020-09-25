use super::component_prelude::*;

/// A `Walker` entity uses `MoveAction::Walk` every frame,
/// in the given direction.
/// Requires the entity to have `Movable` and `MovementAcceleration` components.
/// (All object entities have `Movable`, but `MovementAcceleration` has to be set).
#[derive(Component, Deserialize, Clone)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct Walker {
    pub x: Option<WalkerDirectionX>,
    pub y: Option<WalkerDirectionY>,
}

#[derive(Deserialize, Clone)]
pub enum WalkerDirectionX {
    Left,
    Right,
}

impl WalkerDirectionX {
    pub fn num(&self) -> f32 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
        }
    }
}

#[derive(Deserialize, Clone)]
pub enum WalkerDirectionY {
    Up,
    Down,
}

impl WalkerDirectionY {
    pub fn num(&self) -> f32 {
        match self {
            Self::Up => 1.0,
            Self::Down => -1.0,
        }
    }
}
