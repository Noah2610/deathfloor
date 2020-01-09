use crate::components::prelude::MovementData;

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub movement: MovementData,
}
