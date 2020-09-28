use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionEntityAction;

impl<'a> System<'a> for HandleActionEntityAction {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ActionTrigger<action::EntityAction>>,
        WriteStorage<'a, EntityConfigRegister>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut action_trigger_store,
            mut entity_config_register_store,
        ): Self::SystemData,
    ) {
        for (entity, action_trigger, config_register) in (
            &entities,
            &mut action_trigger_store,
            &mut entity_config_register_store,
        )
            .join()
        {
            for action in action_trigger.drain_actions() {
                match action {
                    action::EntityAction::SwitchVariant(variant_name) => {
                        config_register.add_action(
                            EntityConfigRegisterAction::SwitchVariant(
                                variant_name,
                            ),
                        )
                    }
                    action::EntityAction::PushVariant(variant_name) => {
                        config_register.add_action(
                            EntityConfigRegisterAction::PushVariant(
                                variant_name,
                            ),
                        )
                    }
                    action::EntityAction::PopVariant => config_register
                        .add_action(EntityConfigRegisterAction::PopVariant),
                    action::EntityAction::ApplyComponents => config_register
                        .add_action(
                            EntityConfigRegisterAction::ApplyComponents,
                        ),
                    action::EntityAction::DeleteEntity => {
                        entities.delete(entity).expect(
                            "Couldn't delete entity via \
                             EntityAction::DeleteEntity",
                        );
                    }
                }
            }
        }
    }
}
