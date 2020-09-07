use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(DenseVecStorage)]
pub struct ActionTrigger<A>
where
    A: 'static + Clone + Send + Sync,
{
    actions: Vec<A>,
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
