use crate::components::prelude::*;

#[derive(Default)]
pub struct BulletCreator {
    to_create: Vec<BulletComponents>,
}

impl BulletCreator {
    pub fn add(&mut self, comps: BulletComponents) {
        self.to_create.push(comps);
    }

    pub fn drain(&mut self) -> std::vec::Drain<BulletComponents> {
        self.to_create.drain(..)
    }
}

pub struct BulletComponents {
    pub bullet:        Bullet,
    pub transform:     Transform,
    pub size:          Size,
    pub velocity:      Velocity,
    pub sprite_render: SpriteRender,
}
