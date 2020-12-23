use std::fmt;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize)]
pub enum AnimationKey {
    /// Played when standing on the ground and not moving.
    ///
    /// `Cycle` animation.
    Idle,

    /// Played when standing on the ground and moving left/right.
    ///
    /// `Cycle` animation.
    Walk,

    /// Played once while dying (lifecycle state `Death`).
    ///
    /// `Once` animation.
    Death,

    /// Played once on jump.
    ///
    /// `Once` animation.
    Jump,

    /// Played when moving upwards (positive y velocity).
    ///
    /// `Cycle` animation.
    Rising,

    /// Played when moving downwards (negative y velocity).
    ///
    /// `Cycle` animation.
    Falling,

    /// Custom animations, can be played with `AnimationAction`.
    Custom(String),
}

impl AnimationKey {
    pub fn is_custom(&self) -> bool {
        match self {
            Self::Custom(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for AnimationKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
