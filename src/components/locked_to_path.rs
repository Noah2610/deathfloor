use super::component_prelude::*;
use crate::map_loader::map_data::ObjectPolygon;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct LockedToPath {
    pub path: ObjectPolygon,
}

impl From<ObjectPolygon> for LockedToPath {
    fn from(path: ObjectPolygon) -> Self {
        Self { path }
    }
}
