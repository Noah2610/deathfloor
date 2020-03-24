use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEnemyActionsSystem;

impl<'a> System<'a> for HandleEnemyActionsSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, EventListener>);

    fn run(&mut self, (entities, mut event_listener_store): Self::SystemData) {
        for (_, event_listener) in (&entities, &mut event_listener_store).join()
        {
            for action in event_listener.actions.drain(..) {
                match action {
                    EventAction::Echo(msg) => println!("> {}", msg),
                }
            }
        }
    }
}
