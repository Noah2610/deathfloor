use crate::components::prelude::EnemyType;

#[derive(Debug, Deserialize)]
pub enum ObjectType {
    #[serde(rename = "")]
    None,
    Player,
    Enemy(EnemyType),
}
