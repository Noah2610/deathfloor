(() => {
    function readMap(fileName) {
        return new TileMap();
    }

    function writeMap(map, fileName) {
        const errors = [];

        const tile_size = {
            w: map.tileWidth,
            h: map.tileHeight,
        };
        const outputMap = {
            level: {
                size: {
                    w: map.width * tile_size.w,
                    h: map.height * tile_size.h,
                },
                tile_size,
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
        file.truncate();
        file.write(JSON.stringify(outputMap));
        file.commit();

        return errors.join(", ");
    }

    // Centers the given position, whose origin (0, 0) should be at top-left
    function centerPos(pos, size) {
        return {
            x: pos.x + size.width * 0.5,
            y: pos.y + size.height * 0.5,
        };
    }

    // Inverts the position's y axis, so that the origin point would be bottom-left.
    function invertPosY(pos, mapSize) {
        return {
            x: pos.x,
            y: mapSize.height - pos.y,
        };
    }

    // Converts the given type string into an appropriate type,
    // that can be parsed as an enum with optional fields by serde.
    // Examples:
    //     "Player"
    //         ...converts to...
    //     "Player"
    //
    //     "Enemy(Normal)"
    //         ...converts to...
    //     { "Enemy": "Normal" }
    //
    //     "BigEnemy(Tank, Large)"
    //         ...converts to...
    //     { "Enemy": ["Tank", "Large"] }
    function convertTypeString(typeString) {
        let returnType = typeString;
        const match = typeString.match(/^\s*(\w+)(\((.*)\))?\s*$/) || [];
        const ident = match[1];
        const fieldsString = match[3];
        if (ident) {
            if (fieldsString) {
                const fields = fieldsString.split(/\s*,\s*/);
                if (fields.length === 0) {
                    returnType = {
                        [ident]: null,
                    };
                } else if (fields.length === 1) {
                    returnType = {
                        [ident]: fields[0],
                    };
                } else if (fields.length > 1) {
                    returnType = {
                        [ident]: fields,
                    };
                }
            } else {
                returnType = ident;
            }
        }

        return returnType;
    }

    function getHitboxFrom(objectGroup, layer) {
        const hitboxRects = [];
        const tileSize = {
            width: layer.map.tileWidth,
            height: layer.map.tileHeight,
        };

        for (const object of objectGroup.objects) {
            if (object.shape === MapObject.Rectangle) {
                const pos = invertPosY(
                    centerPos(object.pos, object.size),
                    tileSize
                );
                pos.x -= tileSize.width * 0.5;
                pos.y -= tileSize.height * 0.5;
                const halfSize = {
                    w: object.size.width * 0.5,
                    h: object.size.height * 0.5,
                };
                hitboxRects.push({
                    top: pos.y + halfSize.h,
                    bottom: pos.y - halfSize.h,
                    left: pos.x - halfSize.w,
                    right: pos.x + halfSize.w,
                });
            } else {
                console.warn(
                    "Tile collision objects can only be rectangle shapes, ignoring."
                );
            }
        }
        return hitboxRects;
    }

    function getTilesFromLayer(layer) {
        const output = [];
        const layerSize = {
            width: layer.size.width,
            height: layer.size.height,
        };
        const tileSize = {
            width: layer.map.tileWidth,
            height: layer.map.tileHeight,
        };
        const mapSize = {
            width: layerSize.width * layer.map.tileWidth,
            height: layerSize.height * layer.map.tileHeight,
        };
        const layerProps = layer.properties();

        for (let y = 0; y < layerSize.height; y++) {
            for (let x = 0; x < layerSize.width; x++) {
                const tile = layer.tileAt(x, y);
                if (tile) {
                    const tileOutput = {};
                    const tilesetName = tile.tileset.image
                        .replace(/^.+\/resources\/spritesheets\/tiles\//, "")
                            || "MISSING-TILESET.png";

                    const tileProps = tile.properties();
                    const pos = invertPosY(
                        centerPos(
                            {
                                x: x * tileSize.width,
                                y: y * tileSize.height,
                            },
                            tileSize
                        ),
                        mapSize
                    );

                    tileOutput.id = tile.id;
                    tileOutput.type = convertTypeString(tile.type);
                    tileOutput.ts = tilesetName;
                    tileOutput.pos = pos;
                    tileOutput.props = Object.assign({}, layerProps, tileProps);

                    if (tile.objectGroup) {
                        tileOutput.hitbox = getHitboxFrom(
                            tile.objectGroup,
                            layer
                        );
                    }

                    output.push(tileOutput);
                }
            }
        }

        return output;
    }

    function getObjectsFromLayer(layer) {
        const output = [];

        const mapSize = {
            width: layer.map.width * layer.map.tileWidth,
            height: layer.map.height * layer.map.tileHeight,
        };
        const layerProps = layer.properties();

        for (const object of layer.objects) {
            const objectProps = object.properties();
            const pos = invertPosY(centerPos(object.pos, object.size), mapSize);
            const objectOutput = {
                type: convertTypeString(object.type),
                pos: pos,
                size: {
                    w: object.width,
                    h: object.height,
                },
                props: Object.assign({}, layerProps, objectProps), // { ...layerProps, ...tileProps }
            };

            if (object.polygon && object.polygon.length > 0) {
                objectOutput.polygon = object.polygon.map((point) =>
                    invertPosY(
                        {
                            x: object.pos.x + point.x,
                            y: object.pos.y + point.y,
                        },
                        mapSize
                    )
                );
            }

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
            longName: "deathfloor export script",
        };
        const mapFormat = {
            name: NAME.longName,
            extension: "json",
            read: readMap,
            write: writeMap,
            outputFiles: outputFiles,
        };
        tiled.registerMapFormat(NAME.shortName, mapFormat);
    }

    registerMapFormat();
})();
