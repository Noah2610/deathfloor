use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionEntityAction;

impl<'a> System<'a> for HandleActionEntityAction {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::EntityAction>>,
        WriteStorage<'a, EntityConfigRegister>,
    );

    fn run(
        &mut self,
        (
            mut action_trigger_store,
            mut entity_config_register_store,
        ): Self::SystemData,
    ) {
        for (action_trigger, config_register) in
            (&mut action_trigger_store, &mut entity_config_register_store)
                .join()
        {
            for action in action_trigger.drain_actions() {
                config_register.add_action(action.0);
            }
        }
    }
}
