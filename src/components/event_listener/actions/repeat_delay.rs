use super::ActionType;

/// Repeats the given `action` `loops` amount of times,
/// with each iteration delayed by `delay_ms` amount of milliseconds.
#[derive(Clone, Deserialize)]
pub struct RepeatDelay {
    /// Amount of times the action should be triggered.
    pub loops:    usize,
    /// Delay in milliseconds between each action.
    pub delay_ms: u64,
    /// The action to trigger each iteration.
    pub action:   Box<ActionType>,
}
