use super::ActionType;

/// When triggered, waits the given delay milliseconds,
/// then triggers the wrapped action.
#[derive(Clone, Deserialize)]
pub struct Delay {
    /// Milliseconds to wait, before triggering _action_.
    pub delay_ms: u64,
    /// The action to trigger, once the delay is done.
    pub action:   Box<ActionType>,
}
