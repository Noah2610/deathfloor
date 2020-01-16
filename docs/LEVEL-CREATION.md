# Level Creation with _Tiled_
- [Properties](#properties)
  - [Generic Properties](#generic-properties)
  - [Tile Properties](#tile-properties)
- [Object Types](#object-types)
- [Tilesets](#tilesets)
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
These properties only apply to _tiles_.

- `solid` : `boolean`  
  Tiles with `solid = true` will have collision and be solid to the player.

## Object Types
Every _object_ has to have a type value set.  
Types are always a `string`; these are all valid types:

- `Player`  
  Makes this object the _Player_.  
  The object's position is where the player will spawn in-game.  
  The object's size does not affect the player's in-game size.  
  The player's size is defined in the [`settings.ron`] file.

## Tilesets
Tileset spritesheet images must be PNGs.  
They must be placed under `resources/spritesheets/tiles/`.  
Filenames don't matter, but they should be short but informative.  
(for example: `ground_tiles.png`, `decorations.png`, `grass.png`).

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

[`settings.ron`]: /resources/config/settings.ron
[`sprongen`]:     https://github.com/Noah2610/sprongen
