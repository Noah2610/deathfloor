// resources/entities/player/player.ron

use crate::entity_config::EntityConfig;

pub mod prelude {
    pub use super::PlayerSettings;
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerSettings {
    #[serde(alias = "entity")]
    pub entity_config: Option<EntityConfig>,
}
