use super::EntityConfig;
use super::Merge;
use std::collections::HashMap;

#[derive(Clone, Deserialize, Default)]
#[serde(from = "HashMap<String, EntityConfig>")]
pub struct EntityConfigVariants {
    pub variants: HashMap<String, EntityConfig>,
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
        for (other_variant_name, other_variant) in other.variants {
            if let Some(variant) = self.variants.get_mut(&other_variant_name) {
                variant.merge(other_variant);
            } else {
                self.variants.insert(other_variant_name, other_variant);
            }
        }
    }
}
