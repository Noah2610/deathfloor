use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionControlAction;

impl<'a> System<'a> for HandleActionControlAction {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::ControlAction>>,
        WriteStorage<'a, Controllable>,
    );

    fn run(
        &mut self,
        (mut action_trigger_store, mut controllable_store): Self::SystemData,
    ) {
        for (action_trigger, controllable) in
            (&mut action_trigger_store, &mut controllable_store).join()
        {
            for action in action_trigger.drain_actions() {
                match action {
                    action::ControlAction::SetControllable(is_controllable) => {
                        controllable.is_controllable = is_controllable;
                    }
                }
            }
        }
    }
}
