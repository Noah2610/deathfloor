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

    /// Plays after jump and after `Jump` animation played,
    /// while jumping (while holding down the jump button),
    /// while in-air, and while moving upwards (positive y velocity).
    ///
    /// `Cycle` animation.
    Jumping,

    /// Played when in the air and not jumping.
    ///
    /// `Cycle` animation.
    InAir,

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

// TODO
// `Default` implementation required by `Deserialize` for `AnimationsContainer`.
// This doesn't seem right...
impl Default for AnimationKey {
    fn default() -> Self {
        AnimationKey::Idle
    }
}

impl fmt::Display for AnimationKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Idle => "Idle".to_string(),
            Self::Walk => "Walk".to_string(),
            Self::Death => "Death".to_string(),
            Self::Jump => "Jump".to_string(),
            Self::Jumping => "Jumping".to_string(),
            Self::InAir => "InAir".to_string(),
            Self::Custom(s) => format!("Custom({})", s),
        })
    }
}
