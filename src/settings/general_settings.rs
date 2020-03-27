#[derive(Clone, Deserialize)]
pub struct GeneralSettings {
    pub loader_system: LoaderSystemSettings,
}

#[derive(Clone, Deserialize)]
pub struct LoaderSystemSettings {
    pub use_cache: bool,
}
