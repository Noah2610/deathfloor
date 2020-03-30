use crate::collision_tag::CollisionTag;
use deathframe::physics::query::exp::QueryExpression;

/// Events, which trigger Actions.
#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum EventType {
    /// Triggeres action once, as soon as possible (after entity creation).
    OnSpawn,

    /// __UNIMPLEMENTED__
    /// Triggers action _before_ entity is removed.
    OnDeath,

    /// Triggers action when a collision happens (with entity with `CollisionTag`
    /// that collides with this entity's `CollisionTag`).
    /// Optionally, pass a `QueryExpression`, which when given will be used with
    /// a `FindQuery`, and will only trigger the action if the query matches.
    OnCollision(Option<QueryExpression<CollisionTag>>),

    /// Triggers an action in regular intervals.
    /// Pass an interval delay integer (milliseconds).
    Interval(u64),
}
