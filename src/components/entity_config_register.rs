use super::component_prelude::*;
use crate::settings::prelude::EntityConfig;

#[derive(Clone, Deserialize)]
pub enum EntityConfigRegisterAction {
    SwitchVariant(String),
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct EntityConfigRegister {
    pub config: EntityConfig,
    actions:    Vec<EntityConfigRegisterAction>,
}

impl EntityConfigRegister {
    pub fn new(config: EntityConfig) -> Self {
        Self {
            config,
            actions: Default::default(),
        }
    }
}

impl ActionQueue for EntityConfigRegister {
    type Action = EntityConfigRegisterAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
