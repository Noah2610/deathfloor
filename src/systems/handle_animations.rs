use super::system_prelude::*;

const WALK_ANIM_PADDING: f32 = 8.0;
const JUMPING_ANIM_PADDING: f32 = 0.0;

#[derive(Default)]
pub struct HandleAnimationsSystem;

impl<'a> System<'a> for HandleAnimationsSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Collider<CollisionTag>>,
        ReadStorage<'a, Jumper>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut animations_store,
            velocity_store,
            collider_store,
            jumper_store,
        ): Self::SystemData,
    ) {
        let on_ground_query = {
            use deathframe::physics::query::exp::prelude_variants::*;
            let solid_tag = CollisionTag::from(CollisionLabel::solid());
            And(vec![IsTag(solid_tag), IsSide(Bottom)])
        };

        for (_, animations, velocity, collider_opt, jumper_opt) in (
            &entities,
            &mut animations_store,
            &velocity_store,
            collider_store.maybe(),
            jumper_store.maybe(),
        )
            .join()
        {
            // Only play animation if there is no `Custom` animation playing.
            // `Custom` animations take precedence.
            let should_play_anim = animations
                .current()
                .map(|anim| !anim.is_custom())
                .unwrap_or(true);

            if should_play_anim {
                let (did_jump, is_jumping) = jumper_opt
                    .map(|jumper| (jumper.did_jump, jumper.is_jumping))
                    .unwrap_or((false, false));
                let is_in_air = collider_opt
                    .map(|collider| {
                        collider
                            .query::<FindQuery<_>>()
                            .exp(&on_ground_query)
                            .run()
                            .is_none()
                    })
                    .unwrap_or(false);

                let anim_action = if did_jump
                    && animations.has_animation(&AnimationKey::Jump)
                // JUMP ANIM
                {
                    AnimationAction::Push(AnimationKey::Jump)
                } else if is_jumping
                    && is_in_air
                    && velocity.y > JUMPING_ANIM_PADDING
                    && animations.has_animation(&AnimationKey::Jumping)
                // JUMPING ANIM
                {
                    AnimationAction::Play(AnimationKey::Jumping)
                } else if is_in_air
                    && animations.has_animation(&AnimationKey::InAir)
                // IN-AIR ANIM
                {
                    AnimationAction::Play(AnimationKey::InAir)
                } else {
                    // WALK / IDLE ANIM
                    AnimationAction::Play(match velocity.x {
                        x if x > WALK_ANIM_PADDING => AnimationKey::Walk,
                        x if x < -WALK_ANIM_PADDING => AnimationKey::Walk,
                        _ => AnimationKey::Idle,
                    })
                };

                let _ = match anim_action {
                    AnimationAction::Play(anim) => animations.play(anim),
                    AnimationAction::Push(anim) => animations.push(anim),
                    AnimationAction::Pop => animations.pop().map(|_| ()),
                };
            }
        }
    }
}
