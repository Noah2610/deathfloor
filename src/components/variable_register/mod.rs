use super::component_prelude::*;
use std::collections::HashMap;

pub mod prelude {
    pub use super::VariableRegister;
    pub use super::VariableValue;
}

mod variable_value;

pub use variable_value::VariableValue;

#[derive(Component, Deserialize, Clone, Default)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields, from = "HashMap<String, VariableValue>")]
pub struct VariableRegister {
    pub variables: HashMap<String, VariableValue>,
}

impl From<HashMap<String, VariableValue>> for VariableRegister {
    fn from(variables: HashMap<String, VariableValue>) -> Self {
        Self { variables }
    }
}
