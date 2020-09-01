use super::ActionType;

/// A `PlayerAction` allows any entity
/// to trigger an action on the player.
#[derive(Clone, Deserialize)]
#[serde(from = "ActionType")]
pub struct PlayerAction(pub Box<ActionType>);

impl From<ActionType> for PlayerAction {
    fn from(action: ActionType) -> Self {
        Self(Box::new(action))
    }
}
