use crate::animation_key::AnimationKey;
use crate::collision_tag::CollisionTag;
use crate::components::prelude::{
    LedgeDetectorCorner,
    LedgeDetectorSide,
    LifecycleState,
};
use deathframe::physics::query::exp::prelude::QueryExpression;

/// Events, which trigger Actions.
/// Deserialized from `EventTypeDeser`, which provides some aliases for `EventType`.
/// See `EventTypeDeser` for available aliases.
#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
#[serde(from = "EventTypeDeser")]
pub enum EventType {
    /// Triggeres action every frame where the entity is in the given lifecycle state.
    Lifecycle(LifecycleState),

    /// Triggers action when a collision happens (with entity with `CollisionTag`
    /// that collides with this entity's `CollisionTag`).
    /// Optionally, pass a `QueryExpression`, which when given will be used with
    /// a `FindQuery`, and will only trigger the action if the query matches.
    OnCollision(Option<QueryExpression<CollisionTag>>),

    /// Triggers an action after some time delay.
    /// Trigger after the given milliseconds have passed.
    Delay(u64),

    /// Triggers an action in regular intervals.
    /// Pass an interval delay integer (milliseconds).
    Interval(u64),

    /// Triggers when the entity's `LedgeDetector` detects a ledge.
    OnLedgeDetect(LedgeDetectorCorner, LedgeDetectorSide),

    /// Triggers once for this entity config.
    /// Similar to `OnSpawn`, but also triggers for variants,
    /// when switching to them.
    Init,

    /// Triggers when the given animation ends.
    /// The animation has to be a `Once` animation,
    /// because `Cycle` animations never end.
    OnAnimationEnd(AnimationKey),

    /// Triggers when this entity is interacted with
    /// by an entity with `CanInteract`.
    /// Only available if this entity has the `Interactable` component.
    OnInteract,
}

impl From<EventTypeDeser> for EventType {
    fn from(deser: EventTypeDeser) -> Self {
        use EventType::*;
        use EventTypeDeser as Deser;

        match deser {
            Deser::OnSpawn => Lifecycle(LifecycleState::Spawn),
            Deser::OnDeath => Lifecycle(LifecycleState::Death),

            Deser::Lifecycle(x) => Lifecycle(x),
            Deser::OnCollision(x) => OnCollision(x),
            Deser::Delay(x) => Delay(x),
            Deser::Interval(x) => Interval(x),
            Deser::OnLedgeDetect(corner, side) => OnLedgeDetect(corner, side),
            Deser::Init => Init,
            Deser::OnAnimationEnd(anim) => OnAnimationEnd(anim),
            Deser::OnInteract => OnInteract,
        }
    }
}

/// `EventType` wrapper for deserializing.
/// Provides aliases for certain events.
#[derive(Deserialize, Clone)]
pub enum EventTypeDeser {
    /// Alias for `Lifecycle(Spawn)`
    OnSpawn,
    /// Alias for `Lifecycle(Death)`
    OnDeath,

    Lifecycle(LifecycleState),
    OnCollision(Option<QueryExpression<CollisionTag>>),
    Delay(u64),
    Interval(u64),
    OnLedgeDetect(LedgeDetectorCorner, LedgeDetectorSide),
    Init,
    OnAnimationEnd(AnimationKey),
    OnInteract,
}