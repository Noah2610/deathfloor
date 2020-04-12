initSidebarItems({"derive":[["SystemData","Used to `#[derive]` the trait `SystemData`."]],"enum":[["Axis","Just a plain `Axis` enum with `X` and `Y` variants."],["Texture","Texture wrapper."]],"fn":[["is_entity_loaded",""]],"struct":[["AmethystCamera","Camera struct."],["AssetStorage","An asset storage, storing the actual assets and allocating handles to them."],["AxisIter","An iterator over both axes."],["Confined","Entities that have `Confined` and at least a `Transform`, are confined to the confined `Rect`'s area, with the `ConfineEntitiesSystem`."],["Entity","`Entity` type, as seen by the user."],["EntityComponentInserter","This struct accumulates `InsertionAction`s for entities, and then inserts or removes a certain component from all entities at once, by calling the `run` method. Component to insert/remove has to have `Default` implemented. It can prioritize insertion or removal over the other action. Used with `EntityLoaderSystem`."],["Follow","The `Follow` component makes an entity with a `Transform` follow another entity with a `Transform`. Gives this component to an entity, which should follow another entity."],["Handle","A handle to an asset. This is usually what the user deals with, the actual asset (`A`) is stored in an `AssetStorage`."],["Hidden","Hidden mesh component Useful for entities, that should not be rendered, but stay loaded in memory."],["InputHandler","This struct holds state information about input devices."],["InputManager","Manages input actions. Stores data about which actions are down, up, or being pressed."],["Loadable","Entities which have `Loadable` may be loaded or unloaded (get or remove the `Loaded` component) later on."],["Loaded","Entities which have `Loadable` and `Loaded` will be included in collision detection."],["Loader","`Loader` entities can load `Loadable` entities, whose transforms are within a loading distance from the `Loader` entity's transform."],["Read","Allows to fetch a resource in a system immutably."],["Rect","A `Rect` is simply an area. It has positions bounding sides (top, bottom, left, right)."],["RectBuilder","Builder for `Rect`."],["ResourceId","The id of a [`Resource`], which simply wraps a type id and a \"dynamic ID\". The \"dynamic ID\" is usually just left `0`, and, unless such documentation says otherwise, other libraries will assume that it is always `0`; non-zero IDs are only used for special resource types that are specifically defined in a more dynamic way, such that resource types can essentially be created at run time, without having different static types."],["ScaleOnce","For entities which have `Transform`, `Size`, `SpriteRender`, and `ScaleOnce`, their sprites will be scaled to the entity's size once. After scaling, this component is removed from the entity."],["Size","`Size` is used in multiple places, including collision and scaling."],["SpriteRender","Information for rendering a sprite."],["SpriteSheet","Meta data for a sprite sheet texture."],["SpriteSheetHandles","This is a resource wrapper for amethyst's `SpriteSheet`s. It can load and get `SpriteSheetHandle`s; load them by passing a spritesheet's image file path to an appropriate method and get them by passing their spritesheet's image file name (without extension) to an appropriate method."],["Storage","A wrapper around the masked storage and the generations vector. Can be used for safe lookup of components, insertions and removes. This is what `World::read/write` fetches for the user."],["Time","Frame timing values."],["Transform","Local position, rotation, and scale (from parent if it exists)."],["Transparent","Transparent mesh component"],["World","A [Resource] container, which provides methods to insert, access and manage the contained resources."],["Write","Allows to fetch a resource in a system mutably."]],"trait":[["ActionQueue","The `ActionQueue` can receive and accumulate actions, which can then be consumed at some point."],["ByAxis","Anything implementing the `ByAxis` trait, returns an item through the `by_axis` method, by passing an `&Axis`. This is useful when you have a tuple or similar with two items, where each item represents an axis. You can get the specifc item by indexing with an `Axis`. Here's an example, and what code this would save: ``` use deathframe_core::geo::prelude::{Axis, ByAxis};"],["Join","The purpose of the `Join` trait is to provide a way to access multiple storages at the same time with the merged bit set."],["Merge","Merge types together."],["System","A `System`, executed with a set of required [`Resource`]s."],["SystemData","A static system data that can specify its dependencies at statically (at compile-time). Most system data is a `SystemData`, the `DynamicSystemData` type is only needed for very special setups."]],"type":[["Entities","A wrapper for a read `Entities` resource. Note that this is just `Read<Entities>`, so you can easily use it in your system:"],["Index","An index is basically the id of an `Entity`."],["Point",""],["ReadExpect","Allows to fetch a resource in a system immutably. This will panic if the resource does not exist. Usage of `Read` or `Option<Read>` is therefore recommended."],["ReadStorage","A storage with read access."],["SpriteSheetHandle","An asset handle to sprite sheet metadata."],["TextureHandle",""],["Vector",""],["WriteExpect","Allows to fetch a resource in a system mutably. This will panic if the resource does not exist. Usage of `Write` or `Option<Write>` is therefore recommended."],["WriteStorage","A storage with read and write access."]]});