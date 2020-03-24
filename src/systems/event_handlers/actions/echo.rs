use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionEcho;

impl<'a> System<'a> for HandleActionEcho {
    type SystemData = (Entities<'a>, WriteStorage<'a, EventListener>);

    fn run(&mut self, (entities, mut event_listener_store): Self::SystemData) {
        for (_, event_listener) in (&entities, &mut event_listener_store).join()
        {
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
