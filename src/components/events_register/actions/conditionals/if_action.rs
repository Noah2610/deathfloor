use super::super::ActionType;
use super::Condition;

/// An action that triggers the action under the `then` field
/// if the _condition_ under the `if` field passes.
#[derive(Deserialize, Clone)]
pub struct IfAction {
    #[serde(rename = "if")]
    pub condition: Condition,
    #[serde(rename = "then")]
    pub action:    Box<ActionType>,
    #[serde(rename = "else", default)]
    pub fallback:  Option<Box<ActionType>>,
}
