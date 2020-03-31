pub mod prelude {
    pub use super::CollisionLabel;
    pub use super::CollisionTag;
    pub use super::SolidTag;
}

mod collision_tag;
mod enemy_collides_with;
// mod solid_tag;

#[derive(Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum CollisionLabel {
    Player,
    Tile,
    Jumppad,
    Bullet,
    Enemy,
}

pub use collision_tag::CollisionTag;
pub use enemy_collides_with::EnemyCollidesWith;
pub type SolidTag = CollisionTag;
// pub use solid_tag::SolidTag;

use deathframe::physics::CollisionTag as CTag;
