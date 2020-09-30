mod components;
mod inheritance_chain;
mod variants;

pub mod prelude {
    pub use super::components::{
        EntityConfigComponents,
        EntityConfigComponentsStorages,
    };
    pub use super::variants::EntityConfigVariants;
    pub use super::EntityConfig;
}

use crate::collision_tag::CollisionTagWrapper;
use crate::components::prelude::EventsRegister;
use crate::settings::prelude::AbstractEntitiesSettings;
use components::EntityConfigComponents;
use deathframe::core::components::component_helpers::prelude::Merge;
use inheritance_chain::EntityConfigInheritanceChain;
use std::collections::HashMap;
use variants::EntityConfigVariants;

/// Config for entities.
/// All fields are optional and can be omitted.
#[derive(Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EntityConfig {
    /// Inherit / merge entity configs from the given abstract configs.
    /// Merges configs from left to right, meaning later entries overwrite earlier ones.
    pub inherits:        Option<EntityConfigInheritanceChain>,
    /// List of components to be added to the entity.
    pub components:      Option<EntityConfigComponents>,
    /// Variants for this entity.
    pub variants:        Option<EntityConfigVariants>,
    /// The default variant to use, when no variant prop was set.
    pub default_variant: Option<String>,
    /// Register events/actions.
    pub events:          Option<EventsRegister>,
    /// General collision tag config.
    pub collision_tag:   Option<CollisionTagWrapper>,
    /// Solid collision tag config.
    pub solid_tag:       Option<CollisionTagWrapper>,
}

impl EntityConfig {
    /// Every inherited abstract entity config defined
    /// is merged together here.
    /// All variants' inheritance chains are also merged together.
    /// This function should only be called once.
    pub fn with_inheritance(
        mut self,
        abstract_entities_settings: &AbstractEntitiesSettings,
    ) -> Self {
        // Merge root inheritance chain
        if let Some(inheritance_chain) = &self.inherits {
            self = inheritance_chain
                .generate_entity_config(abstract_entities_settings)
                .merged(self);
        }

        // Merge variants' inheritance chains
        if let Some(variants) = self.variants {
            let new_variants: EntityConfigVariants = variants
                .variants
                .into_iter()
                .map(|(variant_name, variant)| {
                    (
                        variant_name,
                        variant.with_inheritance(abstract_entities_settings),
                    )
                })
                .collect::<HashMap<_, _>>()
                .into();
            self.variants = Some(new_variants);
        }

        self
    }

    /// Merge the given variant name into this entity config.
    /// Returns the variant that has been merged.
    pub fn merge_variant(
        &mut self,
        variant_opt: Option<String>,
    ) -> Option<(String, EntityConfig)> {
        let variant_name = variant_opt.or(self.default_variant.clone());
        let mut merged_variant = None;

        // Merge variant into entity config
        if let Some(variant_name) = variant_name {
            if let Some(variant) = {
                self.variants
                    .as_ref()
                    .and_then(|variants| variants.get(&variant_name).cloned())
            } {
                merged_variant = Some((variant_name, variant.clone()));
                self.merge(variant);
            }
        }

        merged_variant
    }
}

impl Merge for EntityConfig {
    /// `other` takes precedence.
    #[rustfmt::skip]
    fn merge(&mut self, other: Self) {
        *self = Self {
            inherits:        self.inherits.take().merged(other.inherits),
            components:      self.components.take().merged(other.components),
            variants:        self.variants.take().merged(other.variants),
            default_variant: other.default_variant.or(self.default_variant.take()),
            events:          self.events.take().merged(other.events),
            collision_tag:   other.collision_tag.or(self.collision_tag.take()),
            solid_tag:       other.solid_tag.or(self.solid_tag.take()),
        };
    }
}
