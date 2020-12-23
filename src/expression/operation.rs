use super::prelude::*;
use deathframe::amethyst::ecs::Entity;
use std::cmp;

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

    // LOGICAL COMPARISON OPERATIONS
    //
    /// Returns `true` if both given expressions are equal.
    #[serde(alias = "Eq")]
    Equal(Box<Expression>, Box<Expression>),
    /// Returns `true` if the first value is _less than_ the second value.
    #[serde(alias = "Lt")]
    LessThan(Box<Expression>, Box<Expression>),
    /// Returns `true` if the first value is _greater than_ the second value.
    #[serde(alias = "Gt")]
    GreaterThan(Box<Expression>, Box<Expression>),
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
            Self::Equal(a, b) => Value::Bool(
                if let Some(cmp::Ordering::Equal) = a
                    .as_ref()
                    .get(entity, storages)
                    .partial_cmp(&b.as_ref().get(entity, storages))
                {
                    true
                } else {
                    false
                },
            ),
            Self::LessThan(a, b) => Value::Bool(
                if let Some(cmp::Ordering::Less) = a
                    .as_ref()
                    .get(entity, storages)
                    .partial_cmp(&b.as_ref().get(entity, storages))
                {
                    true
                } else {
                    false
                },
            ),
            Self::GreaterThan(a, b) => Value::Bool(
                if let Some(cmp::Ordering::Greater) = a
                    .as_ref()
                    .get(entity, storages)
                    .partial_cmp(&b.as_ref().get(entity, storages))
                {
                    true
                } else {
                    false
                },
            ),
        }
    }
}
