use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionEcho;

impl<'a> System<'a> for HandleActionEcho {
    type SystemData = WriteStorage<'a, ActionTrigger<action::Echo>>;

    fn run(&mut self, mut action_trigger_store: Self::SystemData) {
        for action_trigger in (&mut action_trigger_store).join() {
            for action in action_trigger.drain() {
                println!("{}", action);
            }
        }
    }
}
