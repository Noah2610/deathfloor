use super::ConditionStorages;
use crate::deathframe::components::component_prelude::ByAxis;
use deathframe::amethyst::ecs::Entity;
use deathframe::core::geo::prelude::Axis;
use std::cmp;

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
