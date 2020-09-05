use super::ConditionStorages;
use crate::deathframe::components::component_prelude::ByAxis;
use deathframe::amethyst::ecs::{Entity, Join};
use deathframe::core::geo::prelude::Axis;
use std::cmp;

/// A `ConditionExpression` is used in conditional actions.
/// It can be a _literal value_ when the value is just literally typed in the config,
/// or a _value getter_, which is a placeholder for a value from one of
/// this entity's components (like its position, velocity, or health).
#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ConditionExpression {
    Literal(ConditionExpressionValue),
    Get(ConditionExpressionValueGetter),
    OtherEntityGet(ConditionExpressionOtherEntity),
}

#[derive(Deserialize, Clone, Debug)]
pub enum ConditionExpressionOtherEntity {
    Player(ConditionExpressionValueGetter),
}

impl ConditionExpression {
    pub fn get(
        &self,
        entity: Entity,
        storages: &ConditionStorages,
    ) -> ConditionExpressionValue {
        match self {
            Self::Literal(value) => value.clone(),
            Self::Get(getter) => getter.get(entity, storages),
            Self::OtherEntityGet(ConditionExpressionOtherEntity::Player(
                getter,
            )) => (&storages.entities, &storages.player)
                .join()
                .next()
                .map(|(player, _)| getter.get(player, storages))
                .unwrap_or_else(|| {
                    eprintln!(
                        "[WARNING]\n    Condition player expression couldn't \
                         find a player."
                    );
                    ConditionExpressionValue::Null
                }),
        }
    }
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(untagged)]
pub enum ConditionExpressionValue {
    Null,
    Num(f32),
    Str(String),
    Bool(bool),
}

impl cmp::PartialOrd for ConditionExpressionValue {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        use ConditionExpressionValue::*;

        match (self, other) {
            (Null, Null) => Some(cmp::Ordering::Equal),
            (Num(one), Num(two)) => one.partial_cmp(&two),
            (Str(one), Str(two)) => one.partial_cmp(&two),
            (Bool(one), Bool(two)) => one.partial_cmp(&two),
            (_, _) => {
                eprintln!(
                    "[WARNING]\n    Condition expression values can only be \
                     compared if they are the same type.\n    Got: {:?} and \
                     {:?}",
                    self, other
                );
                None
            }
        }
    }
}

/// A `ConditionExpressionValueGetter` is a sort of placeholder for
/// a specific value on this entity, like its health or velocity.
#[derive(Deserialize, Clone, Debug)]
pub enum ConditionExpressionValueGetter {
    /// Returns the entity's transform position on the given axis as a number.
    Position(Axis),
    /// Returns the entity's velocity on the given axis as a number.
    Velocity(Axis),
    /// Returns the entity's health as a number.
    Health,
}

impl ConditionExpressionValueGetter {
    pub fn get(
        &self,
        entity: Entity,
        storages: &ConditionStorages,
    ) -> ConditionExpressionValue {
        use ConditionExpressionValue as Value;
        match self {
            Self::Position(axis) => {
                if let Some(transform) = storages.transform.get(entity) {
                    let pos = {
                        let trans = transform.translation();
                        (trans.x, trans.y)
                    };
                    Value::Num(pos.by_axis(axis))
                } else {
                    Value::Null
                }
            }

            Self::Velocity(axis) => {
                if let Some(velocity) = storages.velocity.get(entity) {
                    Value::Num(velocity.get(axis))
                } else {
                    Value::Null
                }
            }

            Self::Health => {
                if let Some(health) = storages.health.get(entity) {
                    Value::Num(health.health as f32)
                } else {
                    Value::Null
                }
            }
        }
    }
}
