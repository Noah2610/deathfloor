use deathframe::amethyst::input::BindingTypes;
use std::fmt;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct IngameBindings;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IngameAxisBinding {
    PlayerX,
    PlayerY,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IngameActionBinding {
    PlayerJump,
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

// NOTE: These default implementations don't seem right...
impl Default for IngameAxisBinding {
    fn default() -> Self {
        IngameAxisBinding::PlayerX
    }
}

impl Default for IngameActionBinding {
    fn default() -> Self {
        IngameActionBinding::PlayerJump
    }
}
