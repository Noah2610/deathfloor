use super::component_prelude::*;
use crate::expression::ExpressionValue;
use std::collections::HashMap;

pub mod prelude {
    pub use super::VariableRegister;
}

#[derive(Component, Deserialize, Clone, Default)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields, from = "HashMap<String, ExpressionValue>")]
pub struct VariableRegister {
    pub variables: HashMap<String, ExpressionValue>,
}

impl From<HashMap<String, ExpressionValue>> for VariableRegister {
    fn from(variables: HashMap<String, ExpressionValue>) -> Self {
        Self { variables }
    }
}
