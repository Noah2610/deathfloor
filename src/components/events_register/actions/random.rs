use super::ActionType;

/// Action that randomly triggers one or another action.
#[derive(Clone, Deserialize)]
pub struct Random {
    /// The chance of this action succeeding.
    /// A number between `0.0` and `1.0`.
    pub chance:     f32,
    /// Action to trigger if the random chance succeeded.
    pub on_success: Box<ActionType>,
    /// Action to trigger if the random chance failed.
    /// Optional.
    pub on_failure: Option<Box<ActionType>>,
}
