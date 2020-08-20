use super::component_prelude::*;
use crate::settings::prelude::EntityConfig;

#[derive(Clone, Deserialize)]
pub enum EntityConfigRegisterAction {
    SwitchVariant(String),
    PushVariant(String),
    PopVariant,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct EntityConfigRegister {
    pub root_config:  EntityConfig,
    pub config_stack: Vec<EntityConfig>,
    actions:          Vec<EntityConfigRegisterAction>,
}

impl EntityConfigRegister {
    pub fn new(config: EntityConfig) -> Self {
        Self {
            root_config:  config,
            config_stack: Vec::new(),
            actions:      Default::default(),
        }
    }

    pub fn get_variant(&self, variant_name: &str) -> Option<EntityConfig> {
        self.root_config
            .variants
            .as_ref()
            .and_then(|variants| variants.get(variant_name).cloned())
    }

    pub fn switch_config(&mut self, new_config: EntityConfig) {
        if let Some(last) = self.config_stack.last_mut() {
            *last = new_config;
        } else {
            self.config_stack.push(new_config);
        }
    }

    pub fn push_config(&mut self, new_config: EntityConfig) {
        self.config_stack.push(new_config);
    }

    pub fn pop_config(&mut self) -> Option<EntityConfig> {
        self.config_stack.pop()
    }
}

impl ActionQueue for EntityConfigRegister {
    type Action = EntityConfigRegisterAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
