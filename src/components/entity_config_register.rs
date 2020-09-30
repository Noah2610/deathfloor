use super::component_prelude::*;
use crate::entity_config::prelude::EntityConfig;

type VariantName = String;

#[derive(Clone, Deserialize)]
pub enum EntityConfigRegisterAction {
    SwitchVariant(String),
    PushVariant(String),
    PopVariant,
    ApplyComponents,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct EntityConfigRegister {
    pub root_config:  EntityConfig,
    pub config_stack: Vec<(VariantName, EntityConfig)>,
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

    pub fn switch_config(
        &mut self,
        variant_name: VariantName,
        new_config: EntityConfig,
    ) {
        if let Some(last) = self.config_stack.last_mut() {
            *last = (variant_name, new_config);
        } else {
            self.config_stack.push((variant_name, new_config));
        }
    }

    pub fn push_config(
        &mut self,
        variant_name: VariantName,
        new_config: EntityConfig,
    ) {
        self.config_stack.push((variant_name, new_config));
    }

    pub fn pop_config(&mut self) -> Option<(VariantName, EntityConfig)> {
        self.config_stack.pop()
    }

    pub fn active_variant_name(&self) -> Option<&VariantName> {
        self.config_stack.last().map(|(name, _)| name)
    }
}

impl ActionQueue for EntityConfigRegister {
    type Action = EntityConfigRegisterAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
