//! Expressions are used as conditional values or variables.
//! They can always be evaluated to some form of value.
//! They can be literal values like strings or numbers,
//! or they can be placeholders for component values or for variables values.
//! They are used with entity config `if` conditions and variables.

pub mod prelude {
    pub use super::Expression;
    pub use super::ExpressionComponentValue;
    pub use super::ExpressionStorages;
    pub use super::ExpressionValue;
}

mod component_value;
mod storages;
mod value;

pub use component_value::ExpressionComponentValue;
pub use storages::ExpressionStorages;
pub use value::ExpressionValue;

use deathframe::amethyst::ecs::{Entity, Join};

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum Expression {
    EntityComponentValue(ExpressionEntityComponentValue),
    ComponentValue(ExpressionComponentValue),
    Value(ExpressionValue),
}

#[derive(Deserialize, Clone)]
pub enum ExpressionEntityComponentValue {
    Player(ExpressionComponentValue),
}

impl Expression {
    pub fn get(
        &self,
        entity: Entity,
        storages: &ExpressionStorages,
    ) -> ExpressionValue {
        match self {
            Self::Value(value) => value.clone(),
            Self::ComponentValue(getter) => getter.get(entity, storages),
            Self::EntityComponentValue(
                ExpressionEntityComponentValue::Player(getter),
            ) => (&storages.entities, &storages.player)
                .join()
                .next()
                .map(|(player, _)| getter.get(player, storages))
                .unwrap_or_else(|| {
                    eprintln!(
                        "[WARNING]\n    Condition player expression couldn't \
                         find a player."
                    );
                    ExpressionValue::Null
                }),
        }
    }
}
