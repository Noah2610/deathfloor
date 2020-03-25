use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionGroup;

impl<'a> System<'a> for HandleActionGroup {
    type SystemData = WriteStorage<'a, EventListener>;

    fn run(&mut self, mut event_listener_store: Self::SystemData) {
        for event_listener in (&mut event_listener_store).join() {
            if let Some(actions) =
                event_listener.take_actions(&EventActionType::Group)
            {
                for action in actions {
                    if let EventAction::Group(grouped_actions) = action {
                        for grouped_action in grouped_actions {
                            event_listener.trigger_action(grouped_action);
                        }
                    }
                }
            }
        }
    }
}
