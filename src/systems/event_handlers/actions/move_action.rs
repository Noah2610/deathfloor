use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionMoveAction;

impl<'a> System<'a> for HandleActionMoveAction {
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
                event_listener.take_actions(&EventActionType::MoveAction)
            {
                for action in actions {
                    if let EventAction::MoveAction(move_action) = action {
                        movable.add_action(move_action);
                    }
                }
            }
        }
    }
}
