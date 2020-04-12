initSidebarItems({"enum":[["SoundAction",""]],"struct":[["Animation","Animates an entity with `SpriteRender` frame-by-frame. Iterates through different sprites in the same spritesheet. Each sprite has a duration, in milliseconds, for how long it will be rendered."],["AnimationsContainer","A component, which can hold multiple `Animation`s. It knows which `Animation` is currently active / playing. The `SwitchAnimationsSystem` will switch out the entity's `Animation` component with the active animation from this component."],["BaseFriction","Friction that is applied constantly. You can disable it with the `set_enabled` function."],["Collidable",""],["Collider",""],["Confined","Entities that have `Confined` and at least a `Transform`, are confined to the confined `Rect`'s area, with the `ConfineEntitiesSystem`."],["Follow","The `Follow` component makes an entity with a `Transform` follow another entity with a `Transform`. Gives this component to an entity, which should follow another entity."],["Gravity","Entities with the `Gravity` component are affected by gravity. The gravity's strength is applied to the entity's velocity every frame through the `ApplyGravitySystem`."],["Hidden","Hidden mesh component Useful for entities, that should not be rendered, but stay loaded in memory."],["Hitbox","A `Hitbox` has one or more `Rect` rects, which are collision boxes, relative to this entity's `Transform`. So the `Rect` rects assume the entity's position is at `0, 0`."],["Loadable","Entities which have `Loadable` may be loaded or unloaded (get or remove the `Loaded` component) later on."],["Loaded","Entities which have `Loadable` and `Loaded` will be included in collision detection."],["Loader","`Loader` entities can load `Loadable` entities, whose transforms are within a loading distance from the `Loader` entity's transform."],["ScaleOnce","For entities which have `Transform`, `Size`, `SpriteRender`, and `ScaleOnce`, their sprites will be scaled to the entity's size once. After scaling, this component is removed from the entity."],["Size","`Size` is used in multiple places, including collision and scaling."],["Solid",""],["SoundPlayer",""],["SpriteRender","Information for rendering a sprite."],["Transform","Local position, rotation, and scale (from parent if it exists)."],["Transparent","Transparent mesh component"],["Velocity",""]],"trait":[["ActionQueue","The `ActionQueue` can receive and accumulate actions, which can then be consumed at some point."],["Merge","Merge types together."]]});