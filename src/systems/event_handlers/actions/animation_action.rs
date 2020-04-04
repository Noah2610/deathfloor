use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionAnimationAction;

impl<'a> System<'a> for HandleActionAnimationAction {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::AnimationAction>>,
        WriteStorage<'a, AnimationEditor>,
    );

    fn run(
        &mut self,
        (
            mut action_trigger_store,
            mut animation_editor_store,
        ): Self::SystemData,
    ) {
        for (action_trigger, animation_editor) in
            (&mut action_trigger_store, &mut animation_editor_store).join()
        {
            for action in action_trigger.drain() {
                animation_editor.add_action(action.0);
            }
        }
    }
}
