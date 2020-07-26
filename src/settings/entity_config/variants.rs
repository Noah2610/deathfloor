use super::EntityConfig;
use super::Merge;
use std::collections::HashMap;

#[derive(Clone, Deserialize, Default)]
#[serde(from = "HashMap<String, EntityConfig>")]
pub struct EntityConfigVariants {
    variants: HashMap<String, EntityConfig>,
}

impl EntityConfigVariants {
    pub fn get(&self, name: &str) -> Option<&EntityConfig> {
        self.variants.get(name)
    }
}

impl From<HashMap<String, EntityConfig>> for EntityConfigVariants {
    fn from(variants: HashMap<String, EntityConfig>) -> Self {
        Self { variants }
    }
}

impl Merge for EntityConfigVariants {
    fn merge(&mut self, other: EntityConfigVariants) {
        self.variants.extend(other.variants);
    }
}
