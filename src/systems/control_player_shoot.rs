use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerShootSystem;

impl<'a> System<'a> for ControlPlayerShootSystem {
    type SystemData = (
        ReadExpect<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, CanShoot>,
        ReadStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (input_manager, can_shoot_store, transforms): Self::SystemData,
    ) {
        for (_, transform) in (&can_shoot_store, &transforms).join() {
            let should_shoot = input_manager.is_down(PlayerShoot);
            let facing = Facing::from(transform);

            if should_shoot {
                dbg!("SHOOT!");
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
