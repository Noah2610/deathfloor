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
            (SolidTag::Tile, SolidTag::Tile) => true,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum CollisionTag {
    Player,
    Tile,
}

impl CTag for CollisionTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (CollisionTag::Player, CollisionTag::Tile)
            | (CollisionTag::Tile, CollisionTag::Player) => true,
            (CollisionTag::Player, CollisionTag::Player) => true,
            (CollisionTag::Tile, CollisionTag::Tile) => true,
        }
    }
}

impl Default for CollisionTag {
    fn default() -> Self {
        CollisionTag::Tile
    }
}
