pub mod prelude {
    pub use super::Enemy;
    pub use super::EnemyType;
}

mod enemy_type;

pub use enemy_type::EnemyType;

use super::component_prelude::*;
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
