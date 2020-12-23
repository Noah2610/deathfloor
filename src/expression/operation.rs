use super::prelude::*;
use deathframe::amethyst::ecs::Entity;

/// An `ExpressionOperation` is something that takes an expression,
/// evaluates its value, does something to the `ExpressionValue`,
/// then returns a new `ExpressionValue`.
/// Stuff like arithmetic and logical operations.
#[derive(Deserialize, Clone)]
pub enum ExpressionOperation {
    // LOGICAL OPERATIONS
    //
    /// Returns `true` if _every_ given expression is truthy.
    And(Vec<Expression>),

    /// Returns `true` if _any_ given expression is truthy.
    Or(Vec<Expression>),

    /// Negates the given expression. Will always return a `boolean`.
    /// Returns `true` for falsy values and `false` for truthy values.
    Not(Box<Expression>),
}

impl ExpressionOperation {
    pub(super) fn run(
        &self,
        entity: Entity,
        storages: &ExpressionStorages,
    ) -> ExpressionValue {
        use self::ExpressionValue as Value;
        match self {
            Self::And(exps) => Value::Bool(
                exps.into_iter()
                    .all(|exp| exp.get(entity, storages).is_truthy()),
            ),
            Self::Or(exps) => Value::Bool(
                exps.into_iter()
                    .any(|exp| exp.get(entity, storages).is_truthy()),
            ),
            Self::Not(exp) => {
                Value::Bool(!exp.as_ref().get(entity, storages).is_truthy())
            }
        }
    }
}
