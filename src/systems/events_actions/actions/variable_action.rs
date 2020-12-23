use super::system_prelude::*;
use crate::expression::prelude::ExpressionStorages;

#[derive(Default)]
pub struct HandleActionVariableAction;

impl<'a> System<'a> for HandleActionVariableAction {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ActionTrigger<action::VariableAction>>,
        WriteStorage<'a, UpdateVariableRegister>,
        ExpressionStorages<'a>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut action_trigger_store,
            mut update_variable_register_store,
            expression_stores,
        ): Self::SystemData,
    ) {
        for (entity, action_trigger, update_variable_register) in (
            &entities,
            &mut action_trigger_store,
            &mut update_variable_register_store,
        )
            .join()
        {
            for action in action_trigger.drain_actions() {
                match action {
                    action::VariableAction::Set(name, expression) => {
                        update_variable_register.add_action(
                            UpdateVariableAction::Set(
                                name,
                                expression.get(entity, &expression_stores),
                            ),
                        );
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct HandleUpdateVariableRegister;

impl<'a> System<'a> for HandleUpdateVariableRegister {
    type SystemData = (
        WriteStorage<'a, UpdateVariableRegister>,
        WriteStorage<'a, VariableRegister>,
    );

    fn run(
        &mut self,
        (
            mut update_variable_register_store,
            mut variable_register_store,
        ): Self::SystemData,
    ) {
        for (update_variable_register, variable_register) in (
            &mut update_variable_register_store,
            &mut variable_register_store,
        )
            .join()
        {
            for action in update_variable_register.drain_actions() {
                match action {
                    UpdateVariableAction::Set(name, value) => {
                        variable_register.set(name, value);
                    }
                }
            }
        }
    }
}
