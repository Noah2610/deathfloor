use crate::resources::prelude::SoundType;

#[derive(Clone, Deserialize)]
pub enum SoundAction {
    /// Play the sound associated with the given `SoundType`.
    Play(SoundType),
}
