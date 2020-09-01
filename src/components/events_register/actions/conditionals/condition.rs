pub mod prelude {
    pub use super::Condition;
    pub use super::ConditionExpression;
    pub use super::ConditionGetter;
    pub use super::ConditionStorages;
}

pub use condition_storages::ConditionStorages;

use deathframe::amethyst::ecs::Entity;
use deathframe::core::geo::prelude::Axis;
use std::cmp;

/// A _condition_ used with conditional actions (such as `If`).
/// A condition returns `true` or `false`.
#[derive(Deserialize, Clone)]
pub enum Condition {
    /// Passes if both values are _equal_ (`==`).
    #[serde(alias = "Eq")]
    Equal(ConditionExpression, ConditionExpression),

    /// Passes if the first value is _less than_ the second value (`<`).
    #[serde(alias = "Lt")]
    LessThan(ConditionExpression, ConditionExpression),

    /// Passes if the first value is _greater than_ the second value (`>`).
    #[serde(alias = "Gt")]
    GreaterThan(ConditionExpression, ConditionExpression),
}

impl Condition {
    pub fn passes(&self, entity: Entity, storages: &ConditionStorages) -> bool {
        match self {
            Condition::Equal(one, two) => {
                one.get(entity, storages) == two.get(entity, storages)
            }
            Condition::LessThan(one, two) => {
                one.get(entity, storages) < two.get(entity, storages)
            }
            Condition::GreaterThan(one, two) => {
                one.get(entity, storages) > two.get(entity, storages)
            }
        }
    }
}

/// A `ConditionExpression` is used in conditional actions.
/// It can be a _literal value_ when the value is just literally typed in the config,
/// or a _value getter_, which is a placeholder for a value from one of
/// this entity's components (like its position, velocity, or health).
#[derive(Deserialize, Clone, Debug)]
pub enum ConditionExpression {
    Literal(ConditionValue),
    Get(ConditionGetter),
}

impl ConditionExpression {
    pub fn get(
        &self,
        entity: Entity,
        storages: &ConditionStorages,
    ) -> ConditionValue {
        match self {
            Self::Literal(value) => value.clone(),
            Self::Get(getter) => getter.get(entity, storages),
        }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ConditionValue {
    #[serde(skip)]
    Null,
    Num(f32),
    Str(String),
}

impl cmp::PartialOrd for ConditionValue {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        use ConditionValue::*;
        if let (Num(one), Num(two)) = (self, other) {
            one.partial_cmp(&two)
        } else {
            eprintln!(
                "[WARNING]\n    Comparison conditions (LessThan, GreaterThan) \
                 can only be used with numbers."
            );
            None
        }
    }
}

/// A `ConditionGetter` is a sort of placeholder for
/// a specific value on this entity, like its health or velocity.
#[derive(Deserialize, Clone, Debug)]
pub enum ConditionGetter {
    /// Returns the entity's velocity on the given axis as a number.
    Velocity(Axis),
    /// Returns the entity's health as a number.
    Health,
}

impl ConditionGetter {
    pub fn get(
        &self,
        entity: Entity,
        storages: &ConditionStorages,
    ) -> ConditionValue {
        match self {
            Self::Velocity(axis) => {
                if let Some(velocity) = storages.velocity.get(entity) {
                    ConditionValue::Num(velocity.get(axis))
                } else {
                    ConditionValue::Null
                }
            }
            Self::Health => {
                if let Some(health) = storages.health.get(entity) {
                    ConditionValue::Num(health.health as f32)
                } else {
                    ConditionValue::Null
                }
            }
        }
    }
}

mod condition_storages {
    use crate::components::prelude::*;
    use deathframe::amethyst::ecs::shred::ResourceId;
    use deathframe::amethyst::ecs::{ReadStorage, SystemData, World};

    #[derive(SystemData)]
    pub struct ConditionStorages<'a> {
        pub velocity: ReadStorage<'a, Velocity>,
        pub health:   ReadStorage<'a, Health>,
    }
}
