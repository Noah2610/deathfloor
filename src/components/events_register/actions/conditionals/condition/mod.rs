pub mod prelude {
    pub use super::condition_expression::ConditionExpression;
    pub use super::Condition;
    pub use super::ConditionStorages;
}

mod condition_expression;
mod condition_storages;

pub use condition_storages::ConditionStorages;

use deathframe::amethyst::ecs::Entity;
use prelude::*;

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
