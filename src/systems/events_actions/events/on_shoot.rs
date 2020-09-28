use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventOnShoot;

impl<'a> System<'a> for HandleEventOnShoot {
    type SystemData = (
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, Shooter>,
    );

    fn run(
        &mut self,
        (
            events_register_store,
            mut action_type_trigger_store,
            shooter_store,
        ): Self::SystemData,
    ) {
        for (events_register, action_type_trigger, shooter) in (
            &events_register_store,
            &mut action_type_trigger_store,
            &shooter_store,
        )
            .join()
        {
            if shooter.did_shoot {
                if let Some(action) =
                    events_register.get_action(&EventType::OnShoot)
                {
                    action_type_trigger.add_action(action.clone());
                }
            }
        }
    }
}
