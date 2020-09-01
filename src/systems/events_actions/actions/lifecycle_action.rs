use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionLifecycleAction;

impl<'a> System<'a> for HandleActionLifecycleAction {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::LifecycleAction>>,
        WriteStorage<'a, Lifecycle>,
    );

    fn run(
        &mut self,
        (mut action_trigger_store, mut lifecycle_store): Self::SystemData,
    ) {
        for (action_trigger, lifecycle) in
            (&mut action_trigger_store, &mut lifecycle_store).join()
        {
            for action in action_trigger.drain_actions() {
                match action {
                    action::LifecycleAction::Die => {
                        lifecycle.state = LifecycleState::Death;
                    }
                    action::LifecycleAction::SetState(state) => {
                        lifecycle.state = state;
                    }
                    action::LifecycleAction::Prolong(frames) => {
                        lifecycle.prolong(frames);
                    }
                }
            }
        }
    }
}
