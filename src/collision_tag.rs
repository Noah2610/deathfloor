use crate::components::prelude::EnemyType;
use deathframe::physics::CollisionTag as CTag;

#[derive(Clone, PartialEq)]
pub enum SolidTag {
    Player,
    Tile,
    Enemy(EnemyType),
}

impl CTag for SolidTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (SolidTag::Player, SolidTag::Tile)
            | (SolidTag::Tile, SolidTag::Player) => true,
            (SolidTag::Enemy(_), SolidTag::Tile)
            | (SolidTag::Tile, SolidTag::Enemy(_)) => true,
            (SolidTag::Player, SolidTag::Enemy(_))
            | (SolidTag::Enemy(_), SolidTag::Player) => false,
            (SolidTag::Player, SolidTag::Player) => true,
            (SolidTag::Tile, SolidTag::Tile) => false,
            (SolidTag::Enemy(_), SolidTag::Enemy(_)) => false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum CollisionTag {
    Player,
    Tile,
    Jumppad,
    Bullet,
    Enemy(EnemyType),
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
            (CollisionTag::Player, CollisionTag::Enemy(_))
            | (CollisionTag::Enemy(_), CollisionTag::Player) => true,
            (CollisionTag::Enemy(_), CollisionTag::Bullet)
            | (CollisionTag::Bullet, CollisionTag::Enemy(_)) => true,
            (CollisionTag::Enemy(_), CollisionTag::Jumppad)
            | (CollisionTag::Jumppad, CollisionTag::Enemy(_)) => true,
            _ => false,
        }
    }
}

impl Default for CollisionTag {
    fn default() -> Self {
        CollisionTag::Tile
    }
}
