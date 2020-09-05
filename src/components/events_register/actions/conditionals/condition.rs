pub mod prelude {
    pub use super::Condition;
    pub use super::ConditionExpression;
    pub use super::ConditionGetter;
    pub use super::ConditionStorages;
}

pub use condition_storages::ConditionStorages;

use crate::deathframe::components::component_prelude::ByAxis;
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

    /// Always passes (returns `true`).
    #[serde(alias = "true")]
    True,

    /// Never passes (returns `false`).
    #[serde(alias = "false")]
    False,

    /// Inverts the given condition.
    Not(Box<Condition>),

    /// `And`s the given conditions together.
    /// All given conditions must pass for this condition to pass.
    And(Vec<Condition>),

    /// `Or`s the given conditions together.
    /// Any of the given conditions must pass for this condition to pass.
    Or(Vec<Condition>),
}

impl Condition {
    pub fn passes(&self, entity: Entity, storages: &ConditionStorages) -> bool {
        use Condition::*;
        match self {
            Equal(one, two) => {
                one.get(entity, storages) == two.get(entity, storages)
            }
            LessThan(one, two) => {
                one.get(entity, storages) < two.get(entity, storages)
            }
            GreaterThan(one, two) => {
                one.get(entity, storages) > two.get(entity, storages)
            }
            True => true,
            False => false,
            Not(condition) => !condition.passes(entity, storages),
            And(conditions) => conditions
                .iter()
                .all(|condition| condition.passes(entity, storages)),
            Or(conditions) => conditions
                .iter()
                .any(|condition| condition.passes(entity, storages)),
        }
    }
}

/// A `ConditionExpression` is used in conditional actions.
/// It can be a _literal value_ when the value is just literally typed in the config,
/// or a _value getter_, which is a placeholder for a value from one of
/// this entity's components (like its position, velocity, or health).
#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
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
    Null,
    Num(f32),
    Str(String),
    Bool(bool),
}

impl cmp::PartialOrd for ConditionValue {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        use ConditionValue::*;

        match (self, other) {
            (Null, Null) => Some(cmp::Ordering::Equal),
            (Num(one), Num(two)) => one.partial_cmp(&two),
            (Str(one), Str(two)) => one.partial_cmp(&two),
            (Bool(one), Bool(two)) => one.partial_cmp(&two),
            (_, _) => {
                eprintln!(
                    "[WARNING]\n    Comparison conditions (LessThan, \
                     GreaterThan) can only be used with two values of the \
                     same type."
                );
                None
            }
        }
    }
}

/// A `ConditionGetter` is a sort of placeholder for
/// a specific value on this entity, like its health or velocity.
#[derive(Deserialize, Clone, Debug)]
pub enum ConditionGetter {
    /// Returns the entity's transform position on the given axis as a number.
    Position(Axis),
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
            Self::Position(axis) => {
                if let Some(transform) = storages.transform.get(entity) {
                    let pos = {
                        let trans = transform.translation();
                        (trans.x, trans.y)
                    };
                    ConditionValue::Num(pos.by_axis(axis))
                } else {
                    ConditionValue::Null
                }
            }

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
        pub transform: ReadStorage<'a, Transform>,
        pub velocity:  ReadStorage<'a, Velocity>,
        pub health:    ReadStorage<'a, Health>,
    }
}
