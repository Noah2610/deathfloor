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
    /// Returns `true` if the first value is _less than or equals to_ the second value.
    #[serde(alias = "Le")]
    LessEqual(Box<Expression>, Box<Expression>),
    /// Returns `true` if the first value is _greater than_ the second value.
    #[serde(alias = "Gt")]
    GreaterThan(Box<Expression>, Box<Expression>),
    /// Returns `true` if the first value is _greater than or equals to_ the second value.
    #[serde(alias = "Ge")]
    GreaterEqual(Box<Expression>, Box<Expression>),

    // ARITHMETIC OPERATIONS
    //
    /// Adds the given numbers together.
    /// If any value is not a number, then `Null` is returned
    /// and a warning message is printed to stdout.
    Add(Box<Expression>, Box<Expression>),
    /// Subtracts the second number from the first.
    /// If any value is not a number, then `Null` is returned
    /// and a warning message is printed to stdout.
    Sub(Box<Expression>, Box<Expression>),
    /// Multiplies the given numbers together.
    /// If any value is not a number, then `Null` is returned
    /// and a warning message is printed to stdout.
    Mul(Box<Expression>, Box<Expression>),
    /// Divides the first number by the second.
    /// If any value is not a number, then `Null` is returned
    /// and a warning message is printed to stdout.
    Div(Box<Expression>, Box<Expression>),
    /// Runs the modulo operation on the given numbers (euclidean, `rem_euclid` rust function).
    /// If any value is not a number, then `Null` is returned
    /// and a warning message is printed to stdout.
    Mod(Box<Expression>, Box<Expression>),
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
            Self::LessEqual(a, b) => Value::Bool(
                if let Some(cmp::Ordering::Less) | Some(cmp::Ordering::Equal) =
                    a.as_ref()
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
            Self::GreaterEqual(a, b) => Value::Bool(
                if let Some(cmp::Ordering::Greater)
                | Some(cmp::Ordering::Equal) = a
                    .as_ref()
                    .get(entity, storages)
                    .partial_cmp(&b.as_ref().get(entity, storages))
                {
                    true
                } else {
                    false
                },
            ),
            Self::Add(a, b) => {
                match (a.get(entity, storages), b.get(entity, storages)) {
                    (Value::Num(a), Value::Num(b)) => Value::Num(a + b),
                    (a, b) => {
                        eprintln!(
                            "[WARNING]\n    Can't ADD values that aren't both \
                             numbers:\n    `{:?}` and `{:?}`",
                            a, b,
                        );
                        Value::Null
                    }
                }
            }
            Self::Sub(a, b) => {
                match (a.get(entity, storages), b.get(entity, storages)) {
                    (Value::Num(a), Value::Num(b)) => Value::Num(a - b),
                    (a, b) => {
                        eprintln!(
                            "[WARNING]\n    Can't SUBTRACT values that aren't \
                             both numbers:\n    `{:?}` and `{:?}`",
                            a, b,
                        );
                        Value::Null
                    }
                }
            }
            Self::Mul(a, b) => {
                match (a.get(entity, storages), b.get(entity, storages)) {
                    (Value::Num(a), Value::Num(b)) => Value::Num(a * b),
                    (a, b) => {
                        eprintln!(
                            "[WARNING]\n    Can't MULTIPLY values that aren't \
                             both numbers:\n    `{:?}` and `{:?}`",
                            a, b,
                        );
                        Value::Null
                    }
                }
            }
            Self::Div(a, b) => {
                match (a.get(entity, storages), b.get(entity, storages)) {
                    (Value::Num(a), Value::Num(b)) => Value::Num(a / b),
                    (a, b) => {
                        eprintln!(
                            "[WARNING]\n    Can't DIVIDE values that aren't \
                             both numbers:\n    `{:?}` and `{:?}`",
                            a, b,
                        );
                        Value::Null
                    }
                }
            }
            Self::Mod(a, b) => {
                match (a.get(entity, storages), b.get(entity, storages)) {
                    (Value::Num(a), Value::Num(b)) => {
                        Value::Num(a.rem_euclid(b))
                    }
                    (a, b) => {
                        eprintln!(
                            "[WARNING]\n    Can't run MODULO operation on \
                             values that aren't both numbers:\n    `{:?}` and \
                             `{:?}`",
                            a, b,
                        );
                        Value::Null
                    }
                }
            }
        }
    }
}
