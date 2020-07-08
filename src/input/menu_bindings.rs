// resources/config/menu_bindings.ron

use deathframe::amethyst::input::BindingTypes;
use std::fmt;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct MenuBindings;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MenuAxisBinding {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MenuActionBinding {
    Next,
    Prev,
    Select,
    Quit,
}

impl BindingTypes for MenuBindings {
    type Axis = MenuAxisBinding;
    type Action = MenuActionBinding;
}

impl fmt::Display for MenuAxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for MenuActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// NOTE: These default implementations don't seem right...
// impl Default for MenuAxisBinding {
//     fn default() -> Self {
//         MenuAxisBinding::PlayerX
//     }
// }

// impl Default for MenuActionBinding {
//     fn default() -> Self {
//         MenuActionBinding::PlayerJump
//     }
// }
