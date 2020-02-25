(() => {
    function readMap(fileName) {
        return new TileMap();
    }

    function writeMap(map, fileName) {
        const errors = [];

        const outputMap = {
            level: {
                size: {
                    w: map.width,
                    h: map.height,
                },
                tile_size: {
                    w: map.tileWidth,
                    h: map.tileHeight,
                }
            },
            tiles: [],
            objects: [],
        };

        for (let layerIdx = 0; layerIdx < map.layerCount; layerIdx++) {
            let layer = map.layerAt(layerIdx);
            if (layer.isTileLayer) {
                outputMap.tiles = [].concat(
                    outputMap.tiles,
                    getTilesFromLayer(layer)
                ); // [...outputMap.tiles, ...getTilesFromLayer(layer)]

            } else if (layer.isObjectLayer) {
                outputMap.objects = [].concat(
                    outputMap.objects,
                    getObjectsFromLayer(layer)
                ); // [...outputMap.tiles, ...getObjectsFromLayer(layer)]
            }
        }

        const file = new TextFile(fileName, TextFile.WriteOnly);
        file.write(JSON.stringify(outputMap));
        file.commit();
        file.close();

        return errors.join(", ");
    }

    // Get the centered position with its origin (0, 0) at bottom-left.
    // Assuming that the given pos' origin is at top-left,
    // and that the pos is the top-right corner of the object.
    function getPos(pos, map, sizeMaybe) {
        const size = sizeMaybe || {
            width: map.tileWidth,
            height: map.tileHeight
        };
        const centered = centerPos(pos, size);
        return {
            x: centered.x,
            y: (map.height * size.height) - centered.y,
        };
    }

    // Centers the given position, whose origin (0, 0) should be at top-left
    function centerPos(pos, size) {
        return {
            x: pos.x + (size.width * 0.5),
            y: pos.y + (size.height * 0.5),
        };
    }

    function getHitboxFrom(objectGroup, layer) {
        const hitboxRects = [];
        for (let object of objectGroup.objects) {
            if (object.shape === MapObject.Rectangle) {
                console.log("TODO: Export tile collision objects!"); // TODO
                const tileSize = {
                    width: layer.map.tileWidth,
                    height: layer.map.tileHeight,
                };
                const halfTileSize = {
                    w: tileSize.width * 0.5,
                    h: tileSize.height * 0.5,
                };
                // let pos = centerPos(object.pos, tileSize);
                // pos.y = tileSize.height - pos.y;
                const pos = object.pos;
                const halfSize = {
                    w: object.size.width * 0.5,
                    h: object.size.height * 0.5,
                };
                hitboxRects.push({
                    top:    pos.y - halfSize.h,
                    bottom: pos.y + halfSize.h,
                    left:   pos.x - halfSize.w,
                    right:  pos.x + halfSize.w,
                });
            } else {
                console.warn("Tile collision objects can only be rectangle shapes, ignoring.");
            }
        }
        return hitboxRects;
    }

    function getTilesFromLayer(layer) {
        const output = [];
        const layerSize = {
            w: layer.size.width,
            h: layer.size.height
        };
        const layerProps = layer.properties();
        const tilesetsToAdd = {};

        for (let y = 0; y < layerSize.h; y++) {
            for (let x = 0; x < layerSize.w; x++) {
                const tile = layer.tileAt(x, y);
                if (tile) {
                    const tileOutput = {};
                    if (tile.objectGroup) {
                        tileOutput.hitbox = getHitboxFrom(tile.objectGroup, layer);
                    }

                    const tileset = tile.tileset;
                    const tilesetName = tileset.image.split("/").pop()
                        || "MISSING-TILESET.png";
                    tilesetsToAdd[tilesetName] = tileset;

                    const tileProps = tile.properties();
                    tileOutput.id = tile.id;
                    tileOutput.type = tile.type;
                    tileOutput.ts = tilesetName;
                    tileOutput.pos = getPos({
                        x: x * tile.size.width,
                        y: y * tile.size.height,
                    }, layer.map);
                    tileOutput.props = Object.assign({}, layerProps, tileProps);
                    output.push(tileOutput);
                }
            }
        }

        createTilesetRonsFor(Object.values(tilesetsToAdd));

        return output;
    }

    function createTilesetRonsFor(tilesets) {
        console.log("TODO create RONs");
        // for (let key in tilesets[0]) {
        //     const val = tilesets[0][key];
        //     console.log(`${key} : ${val}`);
        // }
    }

    function getObjectsFromLayer(layer) {
        const output = [];

        const layerProps = layer.properties();

        for (let object of layer.objects) {
            const objectProps = object.properties();
            const pos = getPos(object.pos, layer.map);
            const objectOutput = {
                type: object.type,
                pos: pos,
                size: {
                    w: object.width,
                    h: object.height,
                },
                props: Object.assign({}, layerProps, objectProps), // { ...layerProps, ...tileProps }
            };
            output.push(objectOutput);
        }

        return output;
    }

    function outputFiles(_map, fileName) {
        return [fileName];
    }

    function registerMapFormat() {
        const NAME = {
            shortName: "deathfloor-export",
            longName:  "deathfloor export script",
        };
        const mapFormat = {
            name:        NAME.longName,
            extension:   "json",
            read:        readMap,
            write:       writeMap,
            outputFiles: outputFiles,
        };
        tiled.registerMapFormat(NAME.shortName, mapFormat);
    }

    registerMapFormat();
})();
