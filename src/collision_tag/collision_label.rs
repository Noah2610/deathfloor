#[derive(Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum CollisionLabel {
    Player,
    Tile,
    Solid,
    Jumppad,
    Bullet,
    Enemy,
}
