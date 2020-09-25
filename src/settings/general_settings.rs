// resources/settings/general.ron

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneralSettings {
    pub audio:         AudioSettings,
    pub loader_system: LoaderSystemSettings,
    pub physics:       PhysicsSettings,
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
pub struct PhysicsSettings {
    /// When applying friction, if velocity is equal to or below this value,
    /// then just set the velocity to 0 instead of running friction math.
    pub base_friction_velocity_margin: f32,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DebugSettings {
    pub fps_sample_size: usize,
}
