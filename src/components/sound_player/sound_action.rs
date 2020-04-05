use crate::resources::prelude::SoundType;

#[derive(Clone, Deserialize)]
pub enum SoundAction {
    /// Play the sound associated with the given `SoundType`.
    /// Sound is played with the default volume level (`1.0`).
    Play(SoundType),
    /// Plays the given `SoundType` with the given volume level (`0.0` - `1.0`).
    PlayWithVolume(SoundType, f32),
}
