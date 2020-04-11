#[derive(Clone, Deserialize)]
pub struct GeneralSettings {
    pub audio:         AudioSettings,
    pub loader_system: LoaderSystemSettings,
    pub debug:         DebugSettings,
}

#[derive(Clone, Deserialize)]
pub struct AudioSettings {
    pub sounds_default_volume: f32,
}

#[derive(Clone, Deserialize)]
pub struct LoaderSystemSettings {
    pub use_cache: bool,
}

#[derive(Clone, Deserialize)]
pub struct DebugSettings {
    pub fps_sample_size: usize,
}
