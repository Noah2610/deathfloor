use super::component_prelude::*;
use crate::expression::ExpressionValue;
use std::collections::HashMap;

pub mod prelude {
    pub use super::update_variable_register::{
        UpdateVariableAction,
        UpdateVariableRegister,
    };
    pub use super::VariableRegister;
}

mod update_variable_register;

#[derive(Component, Deserialize, Clone, Default)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields, from = "HashMap<String, ExpressionValue>")]
pub struct VariableRegister {
    variables: HashMap<String, ExpressionValue>,
}

impl VariableRegister {
    pub fn set(&mut self, name: String, value: ExpressionValue) {
        let _ = self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<ExpressionValue> {
        self.variables.get(name).cloned()
    }
}

impl From<HashMap<String, ExpressionValue>> for VariableRegister {
    fn from(variables: HashMap<String, ExpressionValue>) -> Self {
        Self { variables }
    }
}
