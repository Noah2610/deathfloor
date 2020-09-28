use std::fmt;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize)]
pub enum AnimationKey {
    Idle,
    Walk,
    Death,
    Custom(String),
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
            Self::Custom(s) => format!("Custom({})", s),
        })
    }
}
