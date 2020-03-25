use super::CTag;
use super::EnemyCollidesWith;

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
