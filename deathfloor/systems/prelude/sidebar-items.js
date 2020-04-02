initSidebarItems({"struct":[["ApplyBaseFrictionSystem","Constantly applies friction to entities with `BaseFriction`, for each axis. Only if friction is enabled for the axis (see `BaseFriction`)."],["ApplyGravitySystem",""],["CameraOrthoSystem","System that automatically changes the camera matrix according to the settings in the `CameraOrtho` attached to the camera entity."],["ConfineEntitiesSystem","This system confines all entities with `Transform` and `Confined` to the rect defined in `Confined`, taking `Size` into account."],["EntityLoaderSystem","The `EntityLoaderSystem` handles the loading and unloading of entities. Entities with the `Loader` component load entities when they are in range with `Loadable` entities, and `Loadable` entities are unloaded when no `Loader` entities are in range."],["FollowSystem",""],["InputManagerSystem","Handles all the logic for `InputManager`."],["MoveEntitiesSystem","This system is responsible for moving all entities with `Transform` and `Velocity`, by manipulating their `Transform` appropriately. It also handles collision with `Solid` entities; Solid entities may not move into each other."],["PlayAnimationsSystem","Handles the playing of animations for entities with `Animation`."],["ScaleSpritesSystem","This system gets all entities with `Transform`, `Size`, `SpriteRender`, and  `ScaleOnce`, and scales their sprite to their entity's size once; after scaling, the `ScaleOnce` component is removed from the entity."],["SwitchAnimationsSystem","The `SwitchAnimationsSystem` handles entities' `Animation`s with their `AnimationsContainer`s."],["UpdateCollisionsSystem","The `UpdateCollisionsSystem` is in charge of setting collision states for colliding entities. Entities with `CheckCollision` (and with `Collision`) check for collision against other entities with `Collision`. Only checks for entities with either NO `Loadable` and NO `Loaded` components or for entities with `Loadable` AND `Loaded` components; does not check for entities with `Loadable` but NOT `Loaded` components."]]});