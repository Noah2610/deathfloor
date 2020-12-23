use crate::expression::{Expression, ExpressionStorages};
use deathframe::amethyst::ecs::Entity;

/// A _condition_ used with conditional actions (such as `If`).
/// Is simply a wrapper for an `Expression`, which when used
/// as a conditional is converted to a boolean value (truthy/falsy).
#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields, from = "Expression")]
pub struct Condition(Expression);

impl Condition {
    pub fn passes(
        &self,
        entity: Entity,
        storages: &ExpressionStorages,
    ) -> bool {
        self.0.get(entity, storages).is_truthy()
    }
}

impl From<Expression> for Condition {
    fn from(exp: Expression) -> Self {
        Self(exp)
    }
}
