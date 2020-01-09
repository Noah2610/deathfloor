(() => {
    function readMap(fileName) {
        console.log("READ MAP: " + fileName);
        return new TileMap();
    };

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
                ); // [...outputMap.tiles, ...getTilesFromLayer(layer)]
            }
        }

        // TODO save to file
        const file = new TextFile(fileName, TextFile.WriteOnly);
        file.write(JSON.stringify(outputMap));
        file.commit();
        file.close();

        return errors.join(", ");
    };

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
                    const tileset = tile.tileset;
                    const tilesetName = tileset.image.split("/").pop()
                        || "MISSING-TILESET.png";
                    tilesetsToAdd[tilesetName] = tileset;

                    const tileProps = tile.properties();
                    const tileOutput = {
                        id: tile.id,
                        ts: tilesetName,
                        pos: {
                            x: x * tile.size.width,
                            y: y * tile.size.height,
                        },
                        props: Object.assign({}, layerProps, tileProps), // { ...layerProps, ...tileProps }
                    };
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
            const objectOutput = {
                type: object.type,
                pos: {
                    x: object.x,
                    y: object.y,
                },
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
    };

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
