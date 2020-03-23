use super::system_prelude::*;
use deathframe::animation::data::prelude::*;

#[derive(Default)]
pub struct CreateBulletsSystem;

impl<'a> System<'a> for CreateBulletsSystem {
    type SystemData = (Write<'a, BulletCreator>, BulletCreatorStorages<'a>);

    fn run(&mut self, (mut bullet_creator, mut storages): Self::SystemData) {
        for bullet_comps in bullet_creator.drain() {
            let hitbox = Hitbox::from(vec![Rect::from(&bullet_comps.size)]);

            let _entity = storages
                .entities
                .build_entity()
                .with(bullet_comps.bullet, &mut storages.bullet_store)
                .with(bullet_comps.transform, &mut storages.transform_store)
                .with(bullet_comps.size, &mut storages.size_store)
                .with(bullet_comps.velocity, &mut storages.velocity_store)
                .with(
                    bullet_comps.sprite_render,
                    &mut storages.sprite_render_store,
                )
                .with(ScaleOnce::default(), &mut storages.scale_once_store)
                .with(
                    Collider::new(CollisionTag::Bullet),
                    &mut storages.collider_store,
                )
                .with(hitbox, &mut storages.hitbox_store)
                .with(bullet_comps.animation, &mut storages.animation_store)
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
    collider_store:      WriteStorage<'a, Collider<CollisionTag>>,
    hitbox_store:        WriteStorage<'a, Hitbox>,
}
