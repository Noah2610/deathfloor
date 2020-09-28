use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventJumpEvents;

impl<'a> System<'a> for HandleEventJumpEvents {
    type SystemData = (
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, Jumper>,
    );

    fn run(
        &mut self,
        (
            events_register_store,
            mut action_type_trigger_store,
            jumper_store,
        ): Self::SystemData,
    ) {
        for (events_register, action_type_trigger, jumper) in (
            &events_register_store,
            &mut action_type_trigger_store,
            &jumper_store,
        )
            .join()
        {
            if jumper.did_jump {
                if let Some(action) =
                    events_register.get_action(&EventType::OnJump)
                {
                    action_type_trigger.add_action(action.clone());
                }
            }
            if jumper.is_jumping {
                if let Some(action) =
                    events_register.get_action(&EventType::WhileJumping)
                {
                    action_type_trigger.add_action(action.clone());
                }
            }
        }
    }
}
