/// A lifecycle's state.
/// Variants are in-order.
/// So lifecycle state switches occur from top to bottom.
#[derive(Clone, Deserialize)]
pub enum LifecycleState {
    /// Default state, switches to `Spawn` ASAP.
    Initial,
    /// Stays in this state for at least one frame.
    /// Can be _prolonged_.
    Spawn,
    /// Stays in this state until the entity's `Health`
    /// says it's dead. If the entity has no `Health` component,
    /// then the entity will stay in this state indefinitely.
    Alive,
    /// When `Health` determines the entity is dead, then
    /// this is the new state for at least one frame.
    /// Can be _prolonged_.
    Death,
    /// After `Death`, switches to `Despawn`.
    /// `Despawn` means this entity will be deleted ASAP.
    Despawn,
}

impl Default for LifecycleState {
    fn default() -> Self {
        LifecycleState::Initial
    }
}
