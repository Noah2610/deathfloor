use super::component_prelude::*;
use super::lifecycle_state::LifecycleState;

#[derive(Component, Default, Clone)]
#[storage(VecStorage)]
pub struct Lifecycle {
    pub state:     LifecycleState,
    prolong_count: usize,
}

impl Lifecycle {
    pub fn update(&mut self) {
        self.prolong_count =
            self.prolong_count.checked_sub(1).unwrap_or_default();
    }

    /// Prolong this state for at least `frames` more frames.
    /// Only useful for `Spawn` and `Death` states.
    pub fn prolong(&mut self, frames: usize) {
        self.prolong_count = self
            .prolong_count
            .checked_add(frames)
            .unwrap_or(self.prolong_count);
    }

    /// Is the lifecycle manually being prolonged?
    pub fn is_prolonged(&self) -> bool {
        self.prolong_count > 0
    }

    /// Cycle to next state.
    /// Returns an error if the current state is already the final state (`Despawn`),
    /// in which case there is no next state.
    pub fn next_state(&mut self) -> Result<(), String> {
        let new_state = match &self.state {
            LifecycleState::Initial => LifecycleState::Spawn,
            LifecycleState::Spawn => LifecycleState::Alive,
            LifecycleState::Alive => LifecycleState::Death,
            LifecycleState::Death => LifecycleState::Despawn,
            LifecycleState::Despawn => {
                return Err(String::from(
                    "There is no next LifecycleState after `Despawn`",
                ))
            }
        };
        self.state = new_state;
        Ok(())
    }
}
