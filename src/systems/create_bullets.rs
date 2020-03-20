use super::system_prelude::*;

#[derive(Default)]
pub struct CreateBulletsSystem;

impl<'a> System<'a> for CreateBulletsSystem {
    type SystemData = (Write<'a, BulletCreator>, BulletCreatorStorages<'a>);

    fn run(
        &mut self,
        (mut bullet_creator, mut bullet_creator_storages): Self::SystemData,
    ) {
        for bullet_comps in bullet_creator.drain() {
            let _entity = bullet_creator_storages
                .entities
                .build_entity()
                .with(
                    bullet_comps.bullet,
                    &mut bullet_creator_storages.bullet_store,
                )
                .with(
                    bullet_comps.transform,
                    &mut bullet_creator_storages.transform_store,
                )
                .with(
                    bullet_comps.size,
                    &mut bullet_creator_storages.size_store,
                )
                .with(
                    bullet_comps.velocity,
                    &mut bullet_creator_storages.velocity_store,
                )
                .with(
                    bullet_comps.sprite_render,
                    &mut bullet_creator_storages.sprite_render_store,
                )
                .with(
                    ScaleOnce::default(),
                    &mut bullet_creator_storages.scale_once_store,
                )
                .build();
        }
    }
}

#[derive(SystemData)]
pub struct BulletCreatorStorages<'a> {
    entities:            Entities<'a>,
    bullet_store:        WriteStorage<'a, Bullet>,
    transform_store:     WriteStorage<'a, Transform>,
    size_store:          WriteStorage<'a, Size>,
    velocity_store:      WriteStorage<'a, Velocity>,
    scale_once_store:    WriteStorage<'a, ScaleOnce>,
    sprite_render_store: WriteStorage<'a, SpriteRender>,
    animation_store:     WriteStorage<'a, Animation>,
}
