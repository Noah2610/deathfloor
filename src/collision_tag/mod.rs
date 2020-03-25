pub mod prelude {
    pub use super::CollisionTag;
    pub use super::SolidTag;
}

mod collision_tag;
mod enemy_collides_with;
mod solid_tag;

pub use collision_tag::CollisionTag;
pub use enemy_collides_with::EnemyCollidesWith;
pub use solid_tag::SolidTag;

use deathframe::physics::CollisionTag as CTag;
