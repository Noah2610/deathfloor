use crate::expression::Expression;

/// Action that can change a `VariableRegister` variable value.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum VariableAction {
    /// Set the given variable name to the given, evaluated expression's value.
    Set(String, Expression),
}
