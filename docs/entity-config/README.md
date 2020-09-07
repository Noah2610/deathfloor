# Entity Config Documentation
## Overview
An _entity config_ is a generic configuration for entities.  
Some of its features include:
- add _components_ to entity  
  (see [Components])
- program behavior, based on a reactive event/action system  
  (see [Events] and [Actions])
- set the entity's _collision_ and _solid_ tags  
  (see [Collision Tags])
- define entity config _variants_, nested entity configs that  
  can have different components and behavior  
  (see [Variants])

Entity config RON is deserialized into the crate's `EntityConfig` struct.  
See its technical documentation here [`EntityConfig`][docs-EntityConfig].

## Components
Entity config field: `components`

The `components` field contains a list of optional components for this entity.  
See [`EntityComponentsData`][docs-EntityComponentsData] for available components.  

<details>
<summary>
    <strong>Components Example</strong>
</summary>

```ron
(
    components: (
        // See the `Size` component's docs for which fields are required.
        // https://noah2610.github.io/deathfloor/deathfloor/components/prelude/struct.Size.html
        size: (
            w: 16.0,
            h: 24.0,
        ),
        // https://noah2610.github.io/deathfloor/deathfloor/components/prelude/struct.Gravity.html
        gravity: (
            y: -900.0,
        ),
    ),
)
```

Any fields whose value is wrapped with `Option` (in their docs) can be omitted.  
For most components, their docs page shows which fields it needs,  
but for some the way they are configured looks completely different from the docs,  
and there's no easy way to figure out when that is the case from the docs.  
For this I should work on manual documentation for every entity config components.
</details>

## Events
Entity config field: `events`

With _events_ and _actions_ you can program custom behavior for the entity.  
The `events` field in the entity config is a `HashMap` of  
[`EventType`][docs-EventType]/[`ActionType`][docs-ActionType] key/value pairs.  
Each `EventType` key is unique and can only be set once in the `events` field.  
See the [`EventType` docs][docs-EventType] for available events and their descriptions.

When an _event_ occurs in game, then it triggers its _action_.  
See [Actions] to see what actions do.

<details>
<summary>
    <strong>Events Example</strong>
</summary>

```ron
(
    events: { // The `{}` brackets denote a `HashMap`.
    //  EventType                    ActionType
    //  vvvvvvvvvvvvvvvvvvvvvvvvvvv  vvvvvvvvvvvvvvvvvvvvvvvvvvvvv
        OnSpawn:                     Echo("Entity just spawned!"),
        OnCollision(IsTag("Enemy")): Echo("Collision with Enemy!"),
    },
)
```
</details>

## Actions
Used in entity config field: `events`

An _action_ is something that is triggered by _events_.  
An _action_ can do stuff, like change the entity's velocity, play a sound,  
switch _variant_ (see [Variants]), insert components, change health, etc.  
See the [`ActionType` docs][docs-ActionType] for available actions and their descriptions.

When an _event_ occurs, it triggers its _action_.  
Each triggered action is executed in the next frame.  

So if an action triggers another action, the second action is triggered in the frame  
after the first action was triggered. For example, when the `Group` action is triggered,  
it triggers its actions in the frame after the `Group` action was triggered.  
This is useful to keep in mind for certain situations, like the `OnDeath` event.  
When the `OnDeath` event occurs, its action is triggered in the next frame,  
but if the entity is dead in the next frame then the action won't execute.  
In this case, we can work around this by giving the entity a `Death` animation  
in its components' `animations` field (`AnimationContainer` component) ([see this issue comment](https://github.com/Noah2610/deathfloor/issues/56#issuecomment-678921295)).

<details>
<summary>
    <strong>Actions Example</strong>
</summary>

```ron
(
    events: (
        //       Group ActionType
        //       vvvvv
        OnSpawn: Group([
            // Everything in this array is an `ActionType`
            Echo("Just spawned! Gonna do a lil jump!"),
            MoveAction(Jump( y: 150.0 )),
        ]),
        OnCollision(IsTag("Enemy")): Group([
            Echo("Touching an Enemy, gonna take some damage now!"),
            HealthAction(Lose(1)),
        ]),
    ),
)
```
</details>

## Collision Tags
Entity config field for general collision checking: `collision_tag`  
Entity config field for solid collision checking: `solid_tag`

A [`CollisionTag`][docs-CollisionTag] defines with which entities this entity should check collision with.  
The `collision_tag` field's `CollisionTag` is used for general collision checking  
(for example in `OnCollision` events).  
The `solid_tag` `CollisionTag` is only used when moving an entity, to check  
with which entities this entity has solid collision, to see whether it can move there or not.

A `CollisionTag` has the following fields:
- `labels`  
  An array of strings; used with other entities' collision tag's `collides_with` fields.  
  Other entities collide with this entity, if their `collides_with` array mentions one of these labels.
- `collides_with`  
  With which collision labels this entity collides with.  
  This entity collides with entities that have any of these labels specified in their `labels` array.  
  This field is optional, and should be omitted if this entity shouldn't check for collision at all (for example for solid blocks).

<details>
<summary>
    <strong>Collision Tags Example</strong>
</summary>

Some collision rules derived from the below configs:
- The player has solid collision with solid blocks.
- The player checks for collisions with solid blocks, and enemies,  
  so we can check for collisions with these entities with `OnCollision` events.
- The solid block has no solid collisions, because solid blocks don't move.  
  So theoretically if we would move solid blocks, they could pass through everything.
- The solid block generates no collision events at all, because its  
  `collision_tag`'s `collides_with` field is omitted.

#### Player collision tag config example
```ron
(
    collision_tag: (
        labels:        ["Player"],
        collides_with: ["Solid", "Enemy"],
    ),

    solid_tag: (
        labels:        ["Player"],
        collides_with: ["Solid"],
    ),
)
```

#### Solid block collision tag config example
```ron
(
    collision_tag: (
        labels: ["Solid"],
        // Omit field `collides_with`, because solid blocks
        // shouldn't check for collision with anything.
    ),

    solid_tag: (
        labels: ["Solid"],
        // Omit field `collides_with`, because solid blocks don't move and
        // therefor shouldn't check for collisions with other solid entities.
    ),
)
```
</details>

[Components]:     #components
[Events]:         #events
[Actions]:        #actions
[Collision Tags]: #collision-tags
[Variants]:       #variants

[docs-EntityConfig]:         https://noah2610.github.io/deathfloor/deathfloor/settings/entity_config/struct.EntityConfig.html
[docs-EntityComponentsData]: https://noah2610.github.io/deathfloor/deathfloor/settings/entity_config/struct.EntityComponentsData.html
[docs-EventType]:            https://noah2610.github.io/deathfloor/deathfloor/components/events_register/event_type/enum.EventType.html
[docs-ActionType]:           https://noah2610.github.io/deathfloor/deathfloor/components/events_register/actions/enum.ActionType.html
[docs-CollisionTag]:         https://noah2610.github.io/deathfloor/deathfloor/collision_tag/collision_tag/struct.CollisionTag.html
