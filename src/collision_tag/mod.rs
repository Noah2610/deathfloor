pub mod prelude {
    pub use super::CollisionLabel;
    pub use super::CollisionTag;
    pub use super::SolidTag;
}

mod collision_label;
mod collision_tag;

pub type SolidTag = CollisionTag;

pub use collision_label::CollisionLabel;
pub use collision_tag::CollisionTag;
pub use collision_tag::CollisionTagWrapper;

use deathframe::physics::CollisionTag as CTag;
