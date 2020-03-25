use deathframe::physics::CollisionTag as CTag;

pub mod prelude {
    pub use super::CollisionTag;
    pub use super::SolidTag;
}

#[derive(Clone, Hash, Deserialize)]
pub struct EnemyCollidesWith<T: CTag>(pub Option<Vec<T>>);

/// `EnemyCollidesWith` are _always_ equal to each other.
impl<T> PartialEq for EnemyCollidesWith<T>
where
    T: CTag,
{
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T> Eq for EnemyCollidesWith<T> where T: CTag
{
}

/// Collision tags, used for solid collision checking.
/// See the `CollisionTag` implementation to see what collides with what.
#[derive(Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum SolidTag {
    Player,
    Tile,
    Enemy(EnemyCollidesWith<SolidTag>),
}

impl CTag for SolidTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (SolidTag::Player, SolidTag::Tile)
            | (SolidTag::Tile, SolidTag::Player) => true,
            (SolidTag::Enemy(_), SolidTag::Tile)
            | (SolidTag::Tile, SolidTag::Enemy(_)) => true,
            (SolidTag::Enemy(enemy_collides_with), other_tag)
            | (other_tag, SolidTag::Enemy(enemy_collides_with)) => {
                enemy_collides_with
                    .0
                    .as_ref()
                    .map(|collides_with| collides_with.contains(other_tag))
                    .unwrap_or(false)
            }
            (SolidTag::Player, SolidTag::Player) => true,
            (SolidTag::Tile, SolidTag::Tile) => false,
            // (SolidTag::Player, SolidTag::Enemy(_))
            // | (SolidTag::Enemy(_), SolidTag::Player) => false,
            // (SolidTag::Enemy(_), SolidTag::Enemy(_)) => false,
        }
    }
}

/// Collision tags, used for general collision checking.
/// See the `CollisionTag` implementation to see what collides with what.
#[derive(Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum CollisionTag {
    Player,
    Tile,
    Jumppad,
    Bullet,
    Enemy(EnemyCollidesWith<CollisionTag>),
}

impl CTag for CollisionTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (CollisionTag::Player, CollisionTag::Tile)
            | (CollisionTag::Tile, CollisionTag::Player) => true,
            (CollisionTag::Player, CollisionTag::Jumppad)
            | (CollisionTag::Jumppad, CollisionTag::Player) => true,
            (CollisionTag::Player, CollisionTag::Player) => true,
            (CollisionTag::Bullet, CollisionTag::Tile)
            | (CollisionTag::Tile, CollisionTag::Bullet) => true,
            (CollisionTag::Enemy(enemy_collides_with), other_tag)
            | (other_tag, CollisionTag::Enemy(enemy_collides_with)) => {
                enemy_collides_with
                    .0
                    .as_ref()
                    .map(|collides_with| collides_with.contains(other_tag))
                    .unwrap_or(false)
            }
            // (CollisionTag::Player, CollisionTag::Enemy(_))
            // | (CollisionTag::Enemy(_), CollisionTag::Player) => true,
            // (CollisionTag::Enemy(_), CollisionTag::Bullet)
            // | (CollisionTag::Bullet, CollisionTag::Enemy(_)) => true,
            // (CollisionTag::Enemy(_), CollisionTag::Jumppad)
            // | (CollisionTag::Jumppad, CollisionTag::Enemy(_)) => true,
            _ => false,
        }
    }
}

impl Default for CollisionTag {
    fn default() -> Self {
        CollisionTag::Tile
    }
}
