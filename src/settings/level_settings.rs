// resources/settings/levels.ron

#[derive(Clone, Deserialize)]
pub struct LevelSettings {
    pub levels:  Vec<String>,
    pub default: String,
}
