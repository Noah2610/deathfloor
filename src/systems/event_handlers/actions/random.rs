use super::system_prelude::*;
use rand::Rng;

#[derive(Default)]
pub struct HandleActionRandom;

impl<'a> System<'a> for HandleActionRandom {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::Random>>,
        WriteStorage<'a, ActionTypeTrigger>,
    );

    fn run(
        &mut self,
        (
            mut action_trigger_store,
            mut action_type_trigger_store,
        ): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();

        for (action_trigger, action_type_trigger_store) in
            (&mut action_trigger_store, &mut action_type_trigger_store).join()
        {
            for action in action_trigger.drain() {
                let random_num = rng.gen_range(0.0, 1.0);
                if action.chance > random_num {
                    action_type_trigger_store.trigger(*action.on_success);
                } else {
                    if let Some(failure) = action.on_failure {
                        action_type_trigger_store.trigger(*failure);
                    }
                }
            }
        }
    }
}
