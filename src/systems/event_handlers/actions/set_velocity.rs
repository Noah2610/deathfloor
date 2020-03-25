use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionSetVelocity;

impl<'a> System<'a> for HandleActionSetVelocity {
    type SystemData =
        (WriteStorage<'a, EventListener>, WriteStorage<'a, Movable>);

    fn run(
        &mut self,
        (mut event_listener_store, mut movables): Self::SystemData,
    ) {
        for (event_listener, movable) in
            (&mut event_listener_store, &mut movables).join()
        {
            if let Some(actions) =
                event_listener.take_actions(&EventActionType::SetVelocity)
            {
                for action in actions {
                    if let EventAction::SetVelocity { x, y } = action {
                        movable.add_action(MoveAction::SetVelocity {
                            velocity: (x, y),
                        });
                    }
                }
            }
        }
    }
}
