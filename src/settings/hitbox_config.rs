use deathframe::core::geo::prelude::Rect;
use deathframe::physics::components::prelude::Hitbox;

#[derive(Clone, Deserialize)]
pub enum HitboxConfig {
    /// Uses it's Size as the hitbox.
    Size,
    /// Use a custom collection of `Rect`s as the hitbox' rects.
    Custom(Vec<Rect>),
}

impl From<Hitbox> for HitboxConfig {
    fn from(hitbox: Hitbox) -> Self {
        HitboxConfig::Custom(hitbox.into())
    }
}
