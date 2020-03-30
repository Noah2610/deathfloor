use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionGroup;

impl<'a> System<'a> for HandleActionGroup {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::Group>>,
        WriteStorage<'a, ActionTypeTrigger>,
    );

    fn run(
        &mut self,
        (
            mut action_trigger_store,
            mut action_type_trigger_store,
        ): Self::SystemData,
    ) {
        for (action_trigger, action_type_trigger) in
            (&mut action_trigger_store, &mut action_type_trigger_store).join()
        {
            for action in action_trigger.drain() {
                for grouped_action in action.0 {
                    action_type_trigger.trigger(grouped_action);
                }
            }
        }
    }
}
