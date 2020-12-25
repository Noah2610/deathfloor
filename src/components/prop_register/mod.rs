use super::component_prelude::*;
use crate::expression::ExpressionValue;
use std::collections::HashMap;

/// Stores all entity's props,
/// which can then be accessed with expressions.
/// Similar to `VariableRegister`, but these values are immutable.
#[derive(Component, Default, Clone)]
#[storage(DenseVecStorage)]
pub struct PropRegister {
    props: HashMap<String, ExpressionValue>,
}

impl PropRegister {
    pub fn get(&self, key: &str) -> Option<ExpressionValue> {
        self.props.get(key).cloned()
    }
}

impl From<HashMap<String, ExpressionValue>> for PropRegister {
    fn from(props: HashMap<String, ExpressionValue>) -> Self {
        Self { props }
    }
}
