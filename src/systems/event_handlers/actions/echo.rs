use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionEcho;

impl<'a> System<'a> for HandleActionEcho {
    type SystemData = WriteStorage<'a, EventListener>;

    fn run(&mut self, mut event_listener_store: Self::SystemData) {
        for event_listener in (&mut event_listener_store).join() {
            if let Some(actions) =
                event_listener.take_actions(&EventActionType::Echo)
            {
                for action in actions {
                    match action {
                        EventAction::Echo(msg) => println!("> {}", msg),
                    }
                }
            }
        }
    }
}
