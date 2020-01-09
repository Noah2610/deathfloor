use crate::settings::Settings;

pub struct SettingsRes(pub Settings);

impl SettingsRes {
    pub fn new(settings: Settings) -> Self {
        Self(settings)
    }
}
