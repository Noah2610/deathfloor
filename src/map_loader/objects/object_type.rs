use crate::components::prelude::EnemyType;
use deathframe::amethyst::ecs::{Component, VecStorage};

#[derive(Component, Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
#[storage(VecStorage)]
pub enum ObjectType {
    #[serde(rename = "")]
    None,
    Player,
    PlayerBullet,
    Enemy(EnemyType),
    Custom(String),
    CameraPath,
}
