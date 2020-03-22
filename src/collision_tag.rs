use deathframe::physics::CollisionTag as CTag;

#[derive(Clone, PartialEq)]
pub enum SolidTag {
    Player,
    Tile,
}

impl CTag for SolidTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (SolidTag::Player, SolidTag::Tile)
            | (SolidTag::Tile, SolidTag::Player) => true,
            (SolidTag::Player, SolidTag::Player) => true,
            (SolidTag::Tile, SolidTag::Tile) => false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum CollisionTag {
    Player,
    Tile,
    Jumppad,
    Bullet,
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
            _ => false,
        }
    }
}

impl Default for CollisionTag {
    fn default() -> Self {
        CollisionTag::Tile
    }
}
