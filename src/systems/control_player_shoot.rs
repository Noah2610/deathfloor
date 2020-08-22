use super::system_prelude::*;
use crate::map_loader::map_data::prelude::*;
use crate::map_loader::types::ObjectType;
use std::collections::HashMap;

#[derive(Default)]
pub struct ControlPlayerShootSystem;

impl<'a> System<'a> for ControlPlayerShootSystem {
    type SystemData = (
        Write<'a, ObjectSpawner>,
        Read<'a, InputManager<IngameBindings>>,
        WriteStorage<'a, Shooter>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, AnimationEditor>,
        WriteStorage<'a, SoundPlayer<SoundType>>,
    );

    fn run(
        &mut self,
        (
            mut object_spawner,
            input_manager,
            mut shooters,
            transforms,
            mut animation_editor_store,
            mut sound_player_store,
        ): Self::SystemData,
    ) {
        for (shooter, transform, animation_editor_opt, sound_player_opt) in (
            &mut shooters,
            &transforms,
            (&mut animation_editor_store).maybe(),
            (&mut sound_player_store).maybe(),
        )
            .join()
        {
            shooter.cooldown_timer.update().unwrap();
            let should_shoot = input_manager.is_pressed(PlayerShoot)
                && (shooter.cooldown_timer.state.is_finished()
                    || shooter.cooldown_timer.state.is_stopped());
            let facing = Facing::from(transform);

            if should_shoot {
                let bullet_pos = {
                    let trans = transform.translation();
                    (trans.x, trans.y, trans.z + 0.1)
                };

                object_spawner.add(ObjectSpawnData {
                    object: ObjectData {
                        object_type: ObjectType::PlayerBullet,
                        pos:         PosData {
                            x: bullet_pos.0,
                            y: bullet_pos.1,
                        },
                        size:        SizeData { w: 0.0, h: 0.0 },
                        props:       {
                            let mut props = HashMap::new();
                            props.insert(
                                "z".to_string(),
                                (bullet_pos.2 + 0.1).into(),
                            );
                            props.insert(
                                "dir_x".to_string(),
                                facing.mult().into(),
                            );
                            props
                        },
                    },
                });

                shooter.cooldown_timer.start().unwrap();

                if let Some(animation_editor) = animation_editor_opt {
                    animation_editor.add_action(AnimationAction::Push(
                        AnimationKey::Custom("Shoot".into()),
                    ));
                }

                if let Some(sound_player) = sound_player_opt {
                    sound_player.add_action(SoundAction::PlayWithVolume(
                        SoundType::Shoot,
                        0.5,
                    ));
                }
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
