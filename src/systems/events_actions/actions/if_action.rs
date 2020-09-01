use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionIfAction;

impl<'a> System<'a> for HandleActionIfAction {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ActionTrigger<action::conditionals::IfAction>>,
        WriteStorage<'a, ActionTypeTrigger>,
        action::conditionals::ConditionStorages<'a>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut action_trigger_store,
            mut action_type_trigger_store,
            condition_storages,
        ): Self::SystemData,
    ) {
        for (entity, action_trigger, action_type_trigger) in (
            &entities,
            &mut action_trigger_store,
            &mut action_type_trigger_store,
        )
            .join()
        {
            for if_action in action_trigger.drain_actions() {
                if if_action.condition.passes(entity, &condition_storages) {
                    action_type_trigger.add_action(*if_action.action);
                }
            }
        }
    }
}
