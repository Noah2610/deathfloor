use crate::components::prelude::EnemyType;

#[derive(Debug, Deserialize, Clone)]
pub enum ObjectType {
    #[serde(rename = "")]
    None,
    Player,
    PlayerBullet,
    Enemy(EnemyType),
    Custom(String),
    CameraPath,
}
