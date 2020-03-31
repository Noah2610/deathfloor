use crate::components::prelude::EnemyType;

#[derive(Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum CollisionLabel {
    Player,
    Tile,
    Jumppad,
    Bullet,
    Enemy,
}
