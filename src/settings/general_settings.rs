#[derive(Clone, Deserialize)]
pub struct GeneralSettings {
    pub loader_system: LoaderSystemSettings,
    pub debug:         DebugSettings,
}

#[derive(Clone, Deserialize)]
pub struct LoaderSystemSettings {
    pub use_cache: bool,
}

#[derive(Clone, Deserialize)]
pub struct DebugSettings {
    pub fps_sample_size: usize,
}
