(() => {
    const SETTINGS = {
        shortName: "deathfloor-export",
        longName:  "deathfloor export script",
    };

    function registerMapFormat() {
        const readMap = fileName => {
            console.log("READ MAP: " + fileName);
            return new TileMap();
        };

        const writeMap = (map, fileName) => {
            const errors = [];

            const levelSize = {
                w: map.width,
                h: map.height,
            };
            const tileSize = {
                w: map.tileWidth,
                h: map.tileHeight,
            };

            for (let layerIdx = 0; layerIdx < map.layerCount; layerIdx++) {
                let layer = map.layerAt(layerIdx);
                if (layer.isTileLayer) {
                    // TODO do tile layer stuff
                } else if (layer.isObjectLayer) {
                    // TODO do object layer stuff
                }
            }

            // TODO save to file

            return errors.join(", ");
        };

        const outputFiles = (map, fileName) => {
            return [fileName];
        };

        const mapFormat = {
            name:        SETTINGS.longName,
            extension:   "json",
            read:        readMap,
            write:       writeMap,
            outputFiles: outputFiles,
        };
        tiled.registerMapFormat(SETTINGS.shortName, mapFormat);
    }

    function main() {
        registerMapFormat();
    }

    main();
})();
