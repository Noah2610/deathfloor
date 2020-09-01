use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionPlayerAction;

impl<'a> System<'a> for HandleActionPlayerAction {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::PlayerAction>>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, Player>,
    );

    fn run(
        &mut self,
        (
            mut action_trigger_store,
            mut action_type_trigger_store,
            player_store,
        ): Self::SystemData,
    ) {
        for action_trigger in (&mut action_trigger_store).join() {
            for action in action_trigger.drain_actions() {
                let action = *action.0;
                for (_, player_action_type_trigger) in
                    (&player_store, &mut action_type_trigger_store).join()
                {
                    player_action_type_trigger.add_action(action.clone());
                }
            }
        }
    }
}
