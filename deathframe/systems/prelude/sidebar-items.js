initSidebarItems({"struct":[["ApplyBaseFrictionSystem","Constantly applies friction to entities with `BaseFriction`, for each axis. Only if friction is enabled for the axis (see `BaseFriction`)."],["ApplyGravitySystem",""],["ConfineEntitiesSystem","This system confines all entities with `Transform` and `Confined` to the rect defined in `Confined`, taking `Size` into account."],["EntityLoaderSystem","The `EntityLoaderSystem` handles the loading and unloading of entities. Entities with the `Loader` component load entities when they are in range with `Loadable` entities, and `Loadable` entities are unloaded when no `Loader` entities are in range."],["FollowSystem",""],["HandleTakingDamageSystem","This system makes `TakesDamage` entities take damage from `DealsDamage` entities, that it collides with. Dealing damage means losing health."],["InputManagerSystem","Handles all the logic for `InputManager`."],["MoveEntitiesSystem","This system is responsible for moving all entities with `Transform` and `Velocity`, by manipulating their `Transform` appropriately. It also handles collision with `Solid` entities; Solid entities may not move into each other."],["PlayAnimationsSystem","Handles the playing of animations for entities with `Animation`."],["PlaySoundsSystem","Plays queued sounds from `SoundPlayer` components. `SoundAction::Play` sounds are played with the default volume, which can be set with the `with_default_volume` builder function. See the `Default` implementation for the default."],["PrintFpsSystem","Prints the current and average FPS to `stderr`."],["ScaleSpritesSystem","This system gets all entities with `Transform`, `Size`, `SpriteRender`, and  `ScaleOnce`, and scales their sprite to their entity's size once; after scaling, the `ScaleOnce` component is removed from the entity."],["SwitchAnimationsSystem","The `SwitchAnimationsSystem` handles entities' `Animation`s with their `AnimationsContainer`s."],["UpdateCollisionsSystem","The `UpdateCollisionsSystem` is in charge of setting collision states for colliding entities. Entities with `CheckCollision` (and with `Collision`) check for collision against other entities with `Collision`. Only checks for entities with either NO `Loadable` and NO `Loaded` components or for entities with `Loadable` AND `Loaded` components; does not check for entities with `Loadable` but NOT `Loaded` components."],["UpdateHealthSystem","Updates entities' `Health`, via `HealthAction`s from their `HealthActionQueue`."],["UpdateLifecycleSystem","Handles updating of entity's `Lifecycle` component's `LifecycleState`. Will also delete entities when their state switches to `Despawn`."],["UpdateSongPlaybackSystem",""],["UpdateSongVolumeSystem","Handles updating the default audio output sink's volume, used with `Songs`."]]});