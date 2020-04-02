use crate::settings::prelude::EnemyComponentsData;

/// Insert (add or overwrite existing) components.
/// Can be given the same components as
/// in the enemy config's `components` field.
#[derive(Clone, Deserialize)]
#[serde(from = "EnemyComponentsData")]
pub struct InsertComponents(pub EnemyComponentsData);

impl From<EnemyComponentsData> for InsertComponents {
    fn from(components: EnemyComponentsData) -> Self {
        Self(components)
    }
}
