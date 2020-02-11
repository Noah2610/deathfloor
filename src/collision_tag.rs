use deathframe::physics::CollisionTag as CTag;

// NOTE: Use type alias for SolidTag for now,
// in case we ever want to add a proper SolidTag.
pub type SolidTag = CollisionTag;

#[derive(Clone)]
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
