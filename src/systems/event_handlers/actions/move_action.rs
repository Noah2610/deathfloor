use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionMoveAction;

impl<'a> System<'a> for HandleActionMoveAction {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::MoveAction>>,
        WriteStorage<'a, Movable>,
    );

    fn run(
        &mut self,
        (mut action_trigger_store, mut movables): Self::SystemData,
    ) {
        for (action_trigger, movable) in
            (&mut action_trigger_store, &mut movables).join()
        {
            for action in action_trigger.drain_actions() {
                movable.add_action(action.0);
            }
        }
    }
}
