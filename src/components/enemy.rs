use super::component_prelude::*;

#[derive(Deserialize)]
pub enum EnemyType {
    Normal,
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
