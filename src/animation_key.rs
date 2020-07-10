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
