// resources/config/ingame_bindings.ron

use deathframe::amethyst::input::BindingTypes;
use deathframe::core::geo::prelude::{Axis, ByAxis};
use std::fmt;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct IngameBindings;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IngameAxisBinding {
    MoveX,
    MoveY,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IngameActionBinding {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    Jump,
    Shoot,
    Interact,
    ToggleCrouch,

    TogglePause,
    ReloadLevel,

    Custom(String),
}

impl BindingTypes for IngameBindings {
    type Axis = IngameAxisBinding;
    type Action = IngameActionBinding;
}

impl fmt::Display for IngameAxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for IngameActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<A> From<A> for IngameAxisBinding
where
    A: Into<Axis>,
{
    fn from(axis: A) -> Self {
        (IngameAxisBinding::MoveX, IngameAxisBinding::MoveY)
            .by_axis(&axis.into())
    }
}
