use super::system_prelude::*;
use crate::helpers::resource;
use amethyst::core::math::Vector3;

// TODO
const BULLET_Z: f32 = 3.0;

#[derive(Default)]
pub struct ControlPlayerShootSystem;

impl<'a> System<'a> for ControlPlayerShootSystem {
    type SystemData = (
        Write<'a, BulletCreator>,
        ReadExpect<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Shooter>,
        ReadStorage<'a, Transform>,
        WriteExpect<'a, SpriteSheetHandles>,
    );

    fn run(
        &mut self,
        (
            mut bullet_creator,
            input_manager,
            shooters,
            transforms,
            sprite_sheet_handles,
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
                let bullet_transform = {
                    let trans = transform.translation();
                    Transform::from(Vector3::new(trans.x, trans.y, BULLET_Z))
                };
                let bullet_velocity = {
                    let mut velocity: Velocity =
                        shooter.bullet_data.velocity.into();
                    velocity.x *= facing.mult();
                    velocity
                };

                let _bullet = bullet_creator.add(BulletComponents {
                    transform:     bullet_transform,
                    size:          shooter.bullet_data.size.into(),
                    velocity:      bullet_velocity,
                    sprite_render: SpriteRender {
                        sprite_sheet:  bullet_spritesheet_handle.clone(),
                        sprite_number: 0,
                    },
                });
            }
        }
    }
}

enum Facing {
    Left,
    Right,
}

impl Facing {
    fn mult(&self) -> f32 {
        match self {
            Facing::Left => -1.0,
            Facing::Right => 1.0,
        }
    }
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
