use crate::components::prelude::*;
// use crate::systems::system_prelude::*;

#[derive(Default)]
pub struct BulletCreator {
    to_create: Vec<BulletComponents>,
}

impl BulletCreator {
    pub fn add(&mut self, comps: BulletComponents) {
        self.to_create.push(comps);
    }

    // pub fn create(&mut self, storages: &mut BulletCreatorStorages) {
    //     for comps in self.to_create.drain(..) {
    //         let _entity = storages
    //             .entities
    //             .build_entity()
    //             .with(Bullet::default(), &mut storages.bullet_store)
    //             .with(comps.transform, &mut storages.transform_store)
    //             .with(comps.size, &mut storages.size_store)
    //             .with(comps.velocity, &mut storages.velocity_store)
    //             .with(comps.sprite_render, &mut storages.sprite_render_store)
    //             .build();
    //     }
    // }
}

pub struct BulletComponents {
    pub transform:     Transform,
    pub size:          Size,
    pub velocity:      Velocity,
    pub sprite_render: SpriteRender,
}

// #[derive(SystemData)]
// pub struct BulletCreatorStorages<'a> {
//     entities:            Entities<'a>,
//     bullet_store:        WriteStorage<'a, Bullet>,
//     transform_store:     WriteStorage<'a, Transform>,
//     size_store:          WriteStorage<'a, Size>,
//     velocity_store:      WriteStorage<'a, Velocity>,
//     sprite_render_store: WriteStorage<'a, SpriteRender>,
// }
