use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionHealthAction;

impl<'a> System<'a> for HandleActionHealthAction {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::HealthAction>>,
        WriteStorage<'a, HealthEditor>,
    );

    fn run(
        &mut self,
        (mut action_trigger_store, mut health_editor_store): Self::SystemData,
    ) {
        for (action_trigger, health_editor) in
            (&mut action_trigger_store, &mut health_editor_store).join()
        {
            for action in action_trigger.drain() {
                health_editor.add_action(action.0);
            }
        }
    }
}
