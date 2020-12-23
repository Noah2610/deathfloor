use super::system_prelude::*;
use crate::expression::prelude::ExpressionStorages;

#[derive(Default)]
pub struct HandleActionVariableAction;

impl<'a> System<'a> for HandleActionVariableAction {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ActionTrigger<action::VariableAction>>,
        WriteStorage<'a, VariableRegister>,
        ExpressionStorages<'a>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut action_trigger_store,
            mut variable_register_store,
            expression_stores,
        ): Self::SystemData,
    ) {
        for (entity, action_trigger, variable_register) in (
            &entities,
            &mut action_trigger_store,
            &mut variable_register_store,
        )
            .join()
        {
            for action in action_trigger.drain_actions() {
                match action {
                    action::VariableAction::Set(name, expression) => {
                        variable_register.set(
                            name,
                            expression.get(entity, &expression_stores),
                        );
                    }
                }
            }
        }
    }
}
