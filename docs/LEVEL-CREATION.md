# Level Creation with _Tiled_
- [Properties](#properties)
  - [Generic Properties](#generic-properties)
  - [Tile Properties](#tile-properties)
- [Tile Types](#tile-types)
  - [Available Tile Types](#available-tile-types)
- [Object Types](#object-types)
- [Tilesets](#tilesets)
- [Hitboxes](#hitboxes)
  - [Set hitbox via config file](#set-hitbox-via-config-file)
  - [Set hitbox via Tiled collision editor](#set-hitbox-via-tiled-collision-editor)
- [Exporting the map](#exporting-the-map)
  - [Setup the export script](#setup-the-export-script)
  - [Generating RON config files for tileset spritesheets](#generating-ron-config-files-for-tileset-spritesheets)

## Properties
Any properties given to _layers_ are passed down  
to their children _tiles_ or _objects_.  
It is always a good idea to give layers a `z` property,  
instead of tiles/objects directly.

### Generic Properties
These properties apply to all of: _layers_, _tiles_, _objects_.

- `z` : `float`  
  The `z`-position of this tile. Prefer giving its parent _layer_ this property.

### Tile Properties
Applied to tiles in tileset.

- `is_solid` : `boolean`  
  Makes this tile have solid collision, if it also has a hitbox.  
  See the section on [Hitboxes](#hitboxes) for information on how to set hitboxes.

## Tile Types
Tiles can have a `Type` value set in their tileset.  
Types are basically groups of properties, set in a config file,  
instead of having to set the properties for each tile individually.  
Try to always prefer giving tiles a type and setting their properties in the config,  
and only overwrite properties in tiled when needed.  
See types and their configs in [`tiles.ron`].

### Available Tile Types
- `Ground`  
  Tile with solid collision; hitbox is the full tile-size.
- `Jumppad`  
  Non-solid tile, which the player can jump off of.

## Object Types
Every _object_ has to have a type value set.  
Valid types:

- `Player`  
  Makes this object the _Player_.  
  The object's position is where the player will spawn in-game.  
  The object's size does not affect the player's in-game size.  
  The player's size is defined in the [`settings.ron`] file.

## Tilesets
Tileset spritesheet images must be PNGs.  
They must be placed under `resources/spritesheets/tiles/`.  
Filenames don't matter, but they should be short and informative.  
(for example: `ground_tiles.png`, `decorations.png`, `grass.png`).

## Hitboxes
Hitboxes can be given to tiles via their respective config section in [`tiles.ron`].  
Like with other properties, prefer setting hitboxes in the config.  
(Although for complicated hitboxes, this is definitely easier within tiled).  

Note that to make a tile solid, it needs to have a hitbox _and_  
have the `is_solid` property set to `true` (see [Tile Properties](#tile-properties)).

### Set hitbox via config file
Each tile type config can have a `hitbox` field,  
which can hold one of the following values ...
```ron
hitbox: Size,
```
This makes the hitbox be the same as its size (so the full tile rectangle (8x8)).

```ron
hitbox: Custom([
    (
        top:     4.0,
        bottom: -4.0,
        left:   -4.0,
        right:   4.0,
    ),
]),
```
Here we define a custom collision rect.  
We specify each side of the custom rectangle,  
relative to the center of the tile (0.0, 0.0).  
In this case, this would be the same as `hitbox: Size` in the above  
example, if the tile size is `8x8`.  

We can add multiple collision rects to the same hitbox, like so ...
```ron
hitbox: Custom([
    (
        top:     4.0,
        bottom:  0.0,
        left:   -4.0,
        right:   0.0,
    ),
    (
        top:     0.0,
        bottom: -4.0,
        left:    0.0,
        right:   4.0,
    ),
]),
```
Here we have a hitbox, that would look something like this ...
```
+--+
|  |
+--+--+
   |  |
   +--+
```

### Set hitbox via Tiled collision editor
If you need to have a very specific collision,  
that doesn't need to be re-used too much  
(or it's easier to visually configure it), then  
you can use the built-in collision editor in tiled.  

In the tileset, there is a button at the top that  
opens the collision editor.  
In there, you can place rectangles to visually
configure the collision hitbox.  
Note that _only rectangles_ are supported.

## Exporting the map
### Setup the export script
Minimum required _Tiled_ version: `v1.3.0`  

You'll need to add the custom export script / extension to your _Tiled_.  
1) Close _Tiled_ if it's open.
2) Create a new directory (if it doesn't exist yet) at:  
    - on __Linux__: `$HOME/.config/tiled/extensions/`  
      or run:  
      ```
      mkdir -p ~/.config/tiled/extensions
      ```
    - on __Windows__: `C:\Users\<USER>\AppData\Local\Tiled\extensions\`  
      or run:  
      ```
      mkdir %LOCALAPPDATA%\Tiled\extensions
      ```
3) Copy, or better: _link_ the extension script in this repo  
   at `tiled/extensions/export-script.js` into the newly created directory.  
   Or run one of the following commands  
   (replace `<PATH-TO-REPO>` with the path to the root of this project):
   - on __Linux__ run:  
   ```
   ln -s <PATH-TO-REPO>/tiled/extensions/export-script.js $HOME/.config/tiled/extensions/deathfloor-export.js
   ```
   - on __Windows__ run:  
   ```
   mklink %LOCALAPPDATA%\Tiled\extensions\deathfloor-export.js <PATH-TO-REPO>\tiled\extensions\deathfloor-export.js
   ```
4) Open _Tiled_.  
   You can check the _Tiled_ console to see if the script was loaded successfully,  
   by ticking the checkbox under `View -> Views and Toolbars -> Console`.  

If everything's right, then you should be able to export your map with the export script,  
by selecting `deathfloor export script` in the file-type drop-down in the export window (`ctrl+shift+e`).  

__TODO__  
For now, the game loads the level file at `resources/levels/level.json`,  
so be sure to export to that location.

### Generating RON config files for tileset spritesheets
Whenever you add or change a tileset image, you must regenerate the RON config files  
for the tileset spritesheets.  
You can do this with the `gen-rons` script.  
From the root of this project, simply run ...
```
bin/gen-rons
```
to generate the RON files.  
The first time you run this, you will be prompted to install [`sprongen`],  
the CLI app that generates the RON files.  
You need to have `cargo` installed on your system for the script to be able to install [`sprongen`].

[`settings.ron`]: ../resources/config/settings.ron
[`tiles.ron`]:    ../resources/settings/tiles.ron
[`sprongen`]:     https://github.com/Noah2610/sprongen
