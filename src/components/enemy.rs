use super::component_prelude::*;

use std::fmt;

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EnemyType {
    Normal,
}

impl fmt::Display for EnemyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            EnemyType::Normal => "Normal",
        })
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Enemy {
    pub enemy_type: EnemyType,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        Self { enemy_type }
    }
}
