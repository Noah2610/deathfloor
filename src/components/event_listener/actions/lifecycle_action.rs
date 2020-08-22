use deathframe::core::components::prelude::LifecycleState;

/// Actions that change the entity's `Lifecycle` state.
#[derive(Clone, Deserialize)]
pub enum LifecycleAction {
    /// Kills the enemy.
    /// Similar to setting the entity's health to 0.
    Die,
    /// Directly set the lifecycle state.
    /// Examples:
    /// ```ron
    ///     // Make it "spawn" again.
    ///     // Probably no difference to `Alive` at this point.
    ///     SetState(Spawn)
    ///     // Set it back to the main state, like reviving the entity.
    ///     SetState(Alive)
    ///     // Kill the entity.
    ///     // `OnDeath` events will trigger and `Death` animation will play.
    ///     // Will be deleted after that frame.
    ///     SetState(Death)
    /// ```
    SetState(LifecycleState),
}
