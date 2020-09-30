// resources/entities/player/player_bullet.ron

use crate::entity_config::EntityConfig;

pub mod prelude {
    pub use super::PlayerBulletSettings;
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerBulletSettings {
    #[serde(alias = "spritesheet")]
    pub spritesheet_filename: String,
    #[serde(alias = "entity")]
    pub entity_config:        Option<EntityConfig>,
}
