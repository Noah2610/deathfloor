use super::system_prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct HandleActionForeignEntityAction;

impl<'a> System<'a> for HandleActionForeignEntityAction {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::ForeignEntityAction>>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, ObjectType>,
    );

    fn run(
        &mut self,
        (
            mut action_trigger_store,
            mut action_type_trigger_store,
            object_type_store,
        ): Self::SystemData,
    ) {
        let mut actions_to_trigger =
            HashMap::<action::ForeignEntitySelector, Vec<ActionType>>::new();
        for action_trigger in (&mut action_trigger_store).join() {
            for action in action_trigger.drain_actions() {
                actions_to_trigger
                    .entry(action.selector)
                    .or_default()
                    .push(*action.action);
            }
        }
        for (object_type, action_type_trigger) in
            (&object_type_store, &mut action_type_trigger_store).join()
        {
            let selector = action::ForeignEntitySelector {
                object_type: object_type.clone(),
            };
            if let Some(action) = actions_to_trigger.get(&selector) {
                action_type_trigger.add_actions(action.clone());
            }
        }
    }
}
