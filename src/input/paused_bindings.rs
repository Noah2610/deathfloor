use deathframe::amethyst::input::BindingTypes;
use std::fmt;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct PausedBindings;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PausedAxisBinding {
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PausedActionBinding {
    TogglePause,
}

impl BindingTypes for PausedBindings {
    type Axis = PausedAxisBinding;
    type Action = PausedActionBinding;
}

impl fmt::Display for PausedAxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for PausedActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
