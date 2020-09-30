use super::EntityConfig;
use super::Merge;
use crate::settings::prelude::{AbstractEntitiesSettings, AbstractEntityType};

#[derive(Clone, Deserialize)]
#[serde(from = "Vec<AbstractEntityType>")]
pub struct EntityConfigInheritanceChain {
    chain: Vec<AbstractEntityType>,
}

impl EntityConfigInheritanceChain {
    pub fn generate_entity_config(
        &self,
        abstract_entities_settings: &AbstractEntitiesSettings,
    ) -> EntityConfig {
        let mut entity_config = EntityConfig::default();
        for abstract_type in &self.chain {
            if let Some(abstract_settings) =
                abstract_entities_settings.types.get(abstract_type).cloned()
            {
                let mut abstract_entity = abstract_settings.entity;
                if let Some(inheritance_chain) = &abstract_entity.inherits {
                    abstract_entity = inheritance_chain
                        .generate_entity_config(abstract_entities_settings)
                        .merged(abstract_entity);
                }
                entity_config.merge(abstract_entity);
            } else {
                eprintln!(
                    "[WARNING]\n    Tried to inherit undefined abstract \
                     entity config \"{}\"",
                    abstract_type
                );
            }
        }

        entity_config
    }
}

impl From<Vec<AbstractEntityType>> for EntityConfigInheritanceChain {
    fn from(chain: Vec<AbstractEntityType>) -> Self {
        Self { chain }
    }
}

impl Merge for EntityConfigInheritanceChain {
    fn merge(&mut self, mut other: Self) {
        self.chain.append(&mut other.chain);
    }
}
