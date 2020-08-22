// resources/settings/levels.ron

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LevelSettings {
    pub levels: Vec<String>,
}
