use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionSoundAction;

impl<'a> System<'a> for HandleActionSoundAction {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::SoundAction>>,
        WriteStorage<'a, SoundPlayer>,
    );

    fn run(
        &mut self,
        (mut action_trigger_store, mut sound_player_store): Self::SystemData,
    ) {
        for (action_trigger, sound_player) in
            (&mut action_trigger_store, &mut sound_player_store).join()
        {
            for action in action_trigger.drain_actions() {
                sound_player.add_action(action.0);
            }
        }
    }
}
