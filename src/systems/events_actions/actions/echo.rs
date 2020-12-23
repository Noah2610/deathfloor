use super::system_prelude::*;
use crate::expression::prelude::ExpressionStorages;

#[derive(Default)]
pub struct HandleActionEcho;

impl<'a> System<'a> for HandleActionEcho {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ActionTrigger<action::Echo>>,
        ExpressionStorages<'a>,
    );

    fn run(
        &mut self,
        (entities, mut action_trigger_store, storages): Self::SystemData,
    ) {
        for (entity, action_trigger) in
            (&entities, &mut action_trigger_store).join()
        {
            for action in action_trigger.drain_actions() {
                println!("{}", action.value(entity, &storages));
            }
        }
    }
}
