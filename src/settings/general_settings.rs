#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneralSettings {
    pub audio:         AudioSettings,
    pub loader_system: LoaderSystemSettings,
    pub debug:         DebugSettings,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AudioSettings {
    /// Default volume for sounds, when using `SoundAction::Play`.
    pub sounds_default_volume: f32,
    /// Volume for songs.
    pub songs_volume:          f32,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoaderSystemSettings {
    pub use_cache: bool,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DebugSettings {
    pub fps_sample_size: usize,
}
