use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionHealthAction;

impl<'a> System<'a> for HandleActionHealthAction {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::HealthAction>>,
        WriteStorage<'a, HealthActionQueue>,
    );

    fn run(
        &mut self,
        (mut action_trigger_store, mut health_action_queue_store): Self::SystemData,
    ) {
        for (action_trigger, health_action_queu) in
            (&mut action_trigger_store, &mut health_action_queue_store).join()
        {
            for action in action_trigger.drain_actions() {
                health_action_queu.add_action(action.0);
            }
        }
    }
}
