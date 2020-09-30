use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerKillVelocitySystem;

impl<'a> System<'a> for ControlPlayerKillVelocitySystem {
    type SystemData = (
        ReadExpect<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Controllable>,
        ReadStorage<'a, Collider<CollisionTag>>,
        ReadStorage<'a, KillVelocityMin>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            input_manager,
            player_store,
            controllable_store,
            collider_store,
            kill_velocity_min_store,
            mut velocity_store,
        ): Self::SystemData,
    ) {
        let is_not_moving = input_manager
            .axis_value(IngameAxisBinding::MoveX)
            .map(|x| x == 0.0)
            .unwrap_or(false);

        if is_not_moving {
            let query_exp = {
                use deathframe::physics::query::exp::prelude_variants::*;
                And(vec![
                    IsTag(CollisionTag::from(CollisionLabel::solid())),
                    IsSide(Bottom),
                ])
            };
            for (_, controllable, collider, velocity, min_velocity) in (
                &player_store,
                &controllable_store,
                &collider_store,
                &mut velocity_store,
                &kill_velocity_min_store,
            )
                .join()
            {
                if controllable.is_controllable {
                    let is_standing_on_ground = collider
                        .query::<FindQuery<CollisionTag>>()
                        .exp(&query_exp)
                        .run()
                        .is_some();
                    if is_standing_on_ground {
                        let sign = velocity.x.signum();
                        velocity.x =
                            velocity.x.abs().min(min_velocity.min_velocity)
                                * sign;
                    }
                }
            }
        }
    }
}
