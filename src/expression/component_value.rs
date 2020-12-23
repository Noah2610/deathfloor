use super::prelude::*;
use crate::components::prelude::*;
use deathframe::amethyst::ecs::{Entity, Join};
use deathframe::components::component_prelude::ByAxis;
use deathframe::core::geo::prelude::Axis;
use deathframe::physics::query::prelude::{FindQuery, Query, QueryExpression};

/// An `ExpressionComponentValue` is a sort of placeholder for
/// a specific value on an entity, like its health or velocity.
#[derive(Deserialize, Clone)]
pub enum ExpressionComponentValue {
    /// Returns the entity's transform position on the given axis as a number.
    Position(Axis),

    /// Returns the entity's velocity on the given axis as a number.
    Velocity(Axis),

    /// Returns the entity's health as a number.
    Health,

    /// Returns a boolean depending on if the entity's health is at max.
    HasFullHealth,

    /// Returns a string for the facing direction, "Left" or "Right".
    Facing,

    /// Returns a string, for the currently active variant name, if any.
    /// Returns null if no variant is active (only the root entity config).
    Variant,

    /// Returns a boolean depending on if the there is a collision
    /// with the given collision query.
    /// Returns null if the entity has no `Collider`.
    Collision(QueryExpression<CollisionTag>),

    /// Returns a string name for the currently playing animation.
    Animation,
}

impl ExpressionComponentValue {
    pub fn get(
        &self,
        entity: Entity,
        storages: &ExpressionStorages,
    ) -> ExpressionValue {
        use ExpressionValue as Value;
        match self {
            Self::Position(axis) => {
                if let Some(transform) = storages.transform.get(entity) {
                    let pos = {
                        let trans = transform.translation();
                        (trans.x, trans.y)
                    };
                    Value::Num(pos.by_axis(axis))
                } else {
                    Value::Null
                }
            }

            Self::Velocity(axis) => {
                if let Some(velocity) = storages.velocity.get(entity) {
                    Value::Num(velocity.get(axis))
                } else {
                    Value::Null
                }
            }

            Self::Health => {
                if let Some(health) = storages.health.get(entity) {
                    Value::Num(health.health as f32)
                } else {
                    Value::Null
                }
            }

            Self::HasFullHealth => {
                if let Some(health) = storages.health.get(entity) {
                    Value::Bool(health.has_full_health())
                } else {
                    Value::Null
                }
            }

            Self::Facing => {
                if let Some(facing) = storages.facing.get(entity) {
                    Value::Str(facing.to_string())
                } else {
                    Value::Null
                }
            }

            Self::Variant => {
                if let Some(variant_name) = storages
                    .entity_config_register
                    .get(entity)
                    .and_then(|register| register.active_variant_name())
                {
                    Value::Str(variant_name.to_string())
                } else {
                    Value::Null
                }
            }

            Self::Collision(exp) => {
                if let Some(collider) = storages.collider.get(entity) {
                    Value::Bool(
                        collider
                            .query::<FindQuery<CollisionTag>>()
                            .exp(exp)
                            .run()
                            .is_some(),
                    )
                } else {
                    Value::Null
                }
            }

            Self::Animation => {
                if let Some(animation_name) = storages
                    .animations
                    .get(entity)
                    .and_then(|animations| animations.current())
                {
                    Value::Str(animation_name.to_string())
                } else {
                    Value::Null
                }
            }
        }
    }
}
