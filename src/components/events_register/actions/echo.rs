use crate::expression::{Expression, ExpressionStorages};
use deathframe::amethyst::ecs::Entity;

/// Prints the given expression to stdout.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields, from = "Expression")]
pub struct Echo(pub Expression);

impl From<Expression> for Echo {
    fn from(exp: Expression) -> Self {
        Self(exp)
    }
}

impl Echo {
    /// Returns the `Echo` action's string value to be printed.
    pub fn value(
        &self,
        entity: Entity,
        storages: &ExpressionStorages,
    ) -> String {
        let value = self.0.get(entity, storages);
        format!("> {}", value)
    }
}
