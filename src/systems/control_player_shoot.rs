use super::system_prelude::*;
use crate::helpers::resource;

#[derive(SystemData)]
pub struct BulletCreator<'a> {
    entities:            Entities<'a>,
    bullet_store:        WriteStorage<'a, Bullet>,
    transform_store:     WriteStorage<'a, Transform>,
    size_store:          WriteStorage<'a, Size>,
    velocity_store:      WriteStorage<'a, Velocity>,
    sprite_render_store: WriteStorage<'a, SpriteRender>,
}

impl<'a> BulletCreator<'a> {
    fn create_bullet(
        &mut self,
        comps: BulletComponents,
    ) -> amethyst::Result<Entity> {
        let entity_builder = self
            .entities
            .build_entity()
            .with(Bullet::default(), &mut self.bullet_store)
            .with(comps.transform, &mut self.transform_store)
            .with(comps.size, &mut self.size_store)
            .with(comps.velocity, &mut self.velocity_store)
            .with(comps.sprite_render, &mut self.sprite_render_store);
        Ok(entity_builder.build())
    }
}

struct BulletComponents {
    transform:     Transform,
    size:          Size,
    velocity:      Velocity,
    sprite_render: SpriteRender,
}

#[derive(Default)]
pub struct ControlPlayerShootSystem;

impl<'a> System<'a> for ControlPlayerShootSystem {
    type SystemData = (
        ReadExpect<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Shooter>,
        ReadStorage<'a, Transform>,
        WriteExpect<'a, SpriteSheetHandles>,
        BulletCreator<'a>,
    );

    fn run(
        &mut self,
        (
            input_manager,
            shooters,
            transforms,
            sprite_sheet_handles,
            mut bullet_creator,
        ): Self::SystemData,
    ) {
        let bullet_spritesheet_handle = sprite_sheet_handles
            .get(resource("spritesheets/player_bullet.png"))
            .expect(
                "player_bullet.png spritesheet should be loaded at this point",
            );

        for (shooter, transform) in (&shooters, &transforms).join() {
            let should_shoot = input_manager.is_down(PlayerShoot);
            let facing = Facing::from(transform);

            if should_shoot {
                let _bullet = bullet_creator
                    .create_bullet(BulletComponents {
                        transform:     transform.clone(),
                        size:          shooter.bullet_data.size.into(),
                        velocity:      shooter.bullet_data.velocity.into(),
                        sprite_render: SpriteRender {
                            sprite_sheet:  bullet_spritesheet_handle.clone(),
                            sprite_number: 0,
                        },
                    })
                    .unwrap();
            }
        }
    }
}

enum Facing {
    Left,
    Right,
}

impl<'a> From<&'a Transform> for Facing {
    fn from(transform: &'a Transform) -> Self {
        let scale = transform.scale();
        if scale.x.is_sign_positive() {
            Self::Right
        } else {
            Self::Left
        }
    }
}
