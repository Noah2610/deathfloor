use super::ActionType;

/// Groups a list of `ActionType`s together.
/// When this action is triggered, it triggers _all_ of its actions.
#[derive(Clone, Deserialize)]
#[serde(from = "Vec<ActionType>")]
pub struct Group(pub Vec<ActionType>);

impl From<Vec<ActionType>> for Group {
    fn from(actions: Vec<ActionType>) -> Self {
        Self(actions)
    }
}
