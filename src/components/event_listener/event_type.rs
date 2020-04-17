use crate::collision_tag::CollisionTag;
use crate::components::prelude::LifecycleState;
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

    /// Triggers an action in regular intervals.
    /// Pass an interval delay integer (milliseconds).
    Interval(u64),
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
            Deser::Interval(x) => Interval(x),
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
    Interval(u64),
}
