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
    #[deprecated]
    pub fn trigger(&mut self, action: A) {
        self.add_action(action);
    }

    /// Drain all triggered actions.
    #[deprecated]
    pub fn drain(&mut self) -> Drain<A> {
        self.drain_actions()
    }
}

impl<A> ActionQueue for ActionTrigger<A>
where
    A: 'static + Clone + Send + Sync,
{
    type Action = A;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
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
