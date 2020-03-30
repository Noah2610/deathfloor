use super::component_prelude::*;
use std::vec::Drain;

#[derive(Component, Clone, Deserialize)]
#[storage(DenseVecStorage)]
pub struct ActionTrigger<A>
where
    A: 'static + Clone + Send + Sync,
{
    actions: Vec<A>,
}

impl<A> ActionTrigger<A>
where
    A: 'static + Clone + Send + Sync,
{
    /// Trigger an action.
    pub fn trigger(&mut self, action: A) {
        self.actions.push(action);
    }

    /// Drain all triggered actions.
    pub fn drain(&mut self) -> Drain<A> {
        self.actions.drain(..)
    }
}

impl<A> Default for ActionTrigger<A>
where
    A: 'static + Clone + Send + Sync,
{
    fn default() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
}
