# Entity Config Documentation
<details open>
<summary>
    <strong>Table of Contents</strong>
</summary>

- [Overview](#overview)
- [Components](#components)
- [Events](#events)
- [Actions](#actions)
- [Collision Tags](#collision-tags)
- [Variants](#variants)
  - [Use variant](#use-variant)
  - [Default variant](#default-variant)
  - [The variant stack](#the-variant-stack)
  - [Switching variant details](#switching-variant-details)
</details>

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
in its components' `animations` field (`AnimationContainer` component) ([see issue #56's comment](https://github.com/Noah2610/deathfloor/issues/56#issuecomment-678921295)).

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

__Player collision tag config example__
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

__Solid block collision tag config example__
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

## Variants
Entity config field: `variants`

The `variants` field in the entity config is a `HashMap` of  
variant name (string) / `EntityConfig` key/value pairs.  
A variant is simply a nested entity config, that can be used in addition to the root entity config.  
By default, defined entity config variants are simply ignored.  
When using a variant, the variant entity config is merged with the root entity config.

### Use variant
When placing an entity in Tiled, or when spawning an entity with the `SpawnAction` action,  
you can give the entity a `"variant"` prop with the desired variant name as the string value  
(`"variant": "MyVariant"`) to use that variant's entity config.  
When using a variant, the variant entity config is merged with the root entity config.

<details>
<summary>
    <strong>Use variant example</strong>
</summary>

__In Tiled__  
Give the entity object a `"variant"` prop with the variant name.
```
"variant": "MyVariant"
```

__Spawning with `SpawnAction`__  
Give the entity the `"variant"` prop like this.
```ron
SpawnAction(SpawnRelative((
    object: (
        type: Enemy("MyEnemy"), // or `Custom("MyCustomEntity")`
        pos: (x: 0.0, y: 0.0),
        size: (w: 16.0, h: 16.0),
        // Specify the variant in the `props` `HashMap` like so:
        props: {
            "variant": "MyVariant",
        },
    ),
)))
```
</details>

### Default variant
An entity config can have a `default_variant` field with a variant's name as a string.  
If this field is defined, and no `"variant"` prop was given when spawning the entity,  
then this default variant will be applied immediately on spawn.  
If no `"variant"` prop is given and no `default_variant` is set, then no variant will be applied.

### The variant stack
The entity config has a _variant stack_.  
When _switching_ variants, the last variant on the stack (if any) is replaced with the new variant.  
However, you can also _push_ and _pop_ variants; _push_ a variant onto the variant stack  
with the `EntityAction(PushVariant("MyVariant"))` action. Once you've _pushed_ a variant  
onto the stack, you can then _pop_ that variant off to return to the previously applied variant  
(with the `EntityAction(PopVariant)` action).  
When _pushing_ the pushed variant is applied (same as _switching_), and  
when _popping-off_ the previous variant is applied (again, same as _switching_).

<details>
<summary>
    <strong><code>Peeker</code> enemy variant stack example</strong>
</summary>

The variant stack can be useful in very specific circumstances.  
For example, our [`Peeker`](../../resources/settings/enemies/peeker.ron) enemy uses this mechanic.  
The `Peeker` makes heavy use of variants for its general behavior.  
It has variants for _hiding_ (idling), _charging up_ an attack, _cooling down_ after an attack, etc.  
But we also need to be able to tell it in which direction to shoot, which we normally do with a variant,  
but if it is constantly switching variants for its base behavior, we cannot really store  
in which direction it should be shooting.  
In this case we use the variant stack's push/pop/switch very specifically.  
The `Peeker` has `"ShootLeft"` and `"ShootRight"` variants, which we specify when  
spawning/placing the entity. Immediately it _pushes_ its `"Hide"` variant onto the stack,  
because initially it should be hiding. The peeker does what it does, while _switching_ variants,  
but when it's time to _shoot_, instead of switching it _pops-off_ its variant, returning to  
the initally set variant (either `"ShootLeft"` or `"ShootRight"`),  
after which it pushes the `"Hide"` variant onto the stack again, and so forth.
</details>

### Switching variant details
When switching to another variant, there are a couple things to keep in mind:  

The variant's components are inserted in addition to all components the entity already has.  
__When switching from one variant to another, the previous variant's components are not removed.__  

When switching variant, the new events are the root config's events merged with the variant's events.  
In other words, when switching variant, the previous variant's events _are removed_.

__When switching variant, all events' data is cleared__.  
Events store temporary data for stuff like timers (`Interval`, `Delay` actions).  
When switching, this data is cleared, even if it's a root event.  
So for example, root `Interval` events' timers are reset when switching variant.  
[See issue #60.](https://github.com/Noah2610/deathfloor/issues/60)

A variant entity config cannot have another nested `variants` field.  
Or rather it can, but it will never be used. Just don't add variants to a variant.

A variant can have different `collision_tag` and `solid_tag` fields.  
Although this is untested. It should work though, code looks legit.

<details>
<summary>
    <strong><code>Shooter</code> enemy variants example</strong>
</summary>

With variants you can program different behavior for an entity.  
For example, we have a `Shooter` enemy that shoots in regular intervals,  
while moving in a direction. With variants, we can specify in which direction to move.  
So our `Shooter` has `"Left"` and `"Right"` variants, which make the entity  
move left or right, respectively.  
When placing the `Shooter` in Tiled we give it an initial variant to specify  
in which direction it should initially start moving.  
We've configured the `"Left"` variant to move left, and to _switch_ to the `"Right"` variant  
when it collides with a wall or it detects a ledge to the left; and vice-versa.  
With the `EntityAction(SwitchVariant("VARIANT_NAME"))` action we can switch variants at runtime.

Our `Shooter` enemy entity config (only relevant parts).  
[Full `Shooter` config](../../resources/settings/enemies/shooter.ron).
```ron
(
    /* ... */

    entity: (
        components: (/* ... */),
        events: {/* ... */},

        variants: {
            "Left": (
                // Components to insert for the "Left" variant.
                components: (
                    // Move left with the walker component.
                    walker: (
                        x: -500.0,
                    ),
                ),

                // Events only for the "Left" variant.
                events: {
                    // Switch to "Right" variant when detecting a ledge to the left.
                    OnLedgeDetect(BottomLeft, Bottom):
                        EntityAction(SwitchVariant("Right")),

                    // Switch to "Right" variant when colliding with a solid entity to the left.
                    OnCollision(And([
                            IsTag("Solid"),
                            IsState(Enter),
                            IsSide(Left),
                        ])): EntityAction(SwitchVariant("Right")),

                    // Shoot to the left.
                    Interval(1000): SpawnAction(SpawnRelative((
                        object: (
                            type: Custom("ShooterBullet"),
                            pos: (x: -24.0, y: -4.0),
                            size: (w: 16.0, h: 16.0),
                            props: {
                                "variant": "Left",
                            },
                        ),
                    ))),
                },
            ),

            "Right": (
                // Components to insert for the "Right" variant.
                components: (
                    // Move right with the walker component.
                    walker: (
                        x: 500.0,
                    ),
                ),

                // Events only for the "Right" variant.
                events: {
                    // Switch to "Left" variant when detecting a ledge to the right.
                    OnLedgeDetect(BottomRight, Bottom):
                        EntityAction(SwitchVariant("Left")),

                    // Switch to "Left" variant when colliding with a solid entity to the right.
                    OnCollision(And([
                            IsTag("Solid"),
                            IsState(Enter),
                            IsSide(Right),
                        ])): EntityAction(SwitchVariant("Left")),

                    // Shoot to the right.
                    Interval(1000): SpawnAction(SpawnRelative((
                        object: (
                            type: Custom("ShooterBullet"),
                            pos: (x: 24.0, y: -4.0),
                            size: (w: 16.0, h: 16.0),
                            props: {
                                "variant": "Right",
                            },
                        ),
                    ))),
                },
            ),
        },

        collision_tag: (/* ... */),
        solid_tag: (/* ... */),
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
