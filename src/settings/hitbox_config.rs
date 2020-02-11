use deathframe::core::geo::prelude::Rect;

#[derive(Clone, Deserialize)]
pub enum HitboxConfig {
    /// Uses it's Size as the hitbox.
    Size,
    /// Use a custom collection of `Rect`s as the hitbox' rects.
    Custom(Vec<Rect>),
}
