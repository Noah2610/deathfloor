# Documentation
- [DESIGN](./DESIGN.md)
- [ROADMAP](./ROADMAP.md)
- [FEATURES](./FEATURES.md)
- [STAGES](./STAGES.md)
- [LEVEL-CREATION](./LEVEL-CREATION.md)
- [ENTITY-CONFIG](./entity-config/README.md)

# Concept Art
[`concept-art/`](./concept-art)

# Development
## Level selection
When running the game from the command-line with `bin/run` (or `cargo run`, but always prefer `bin/run`),  
add the level filename as the first argument to load that level.

__Examples__:
```
bin/run level.json
bin/run testlevel5.json
```

Only filename, not path to file.  
Extension necessary.

## Reload level ingame
Press `R` to reload the current level ingame.  
(see `ingame_bindings.ron` config file to set other binding).

Requires `debug` feature (use `bin/run`)

## Generating RON files for spritesheets
Run the `gen-rons` script to generate RON config files for  
all tile spritesheets under `resources/spritesheets/tiles/` ...
```
bin/gen-rons
```

Any arguments are passed to the underlying `sprongen` command.  
(Run `bin/gen-rons --help` for more info)

Uses and depends on the [`sprongen`][sprongen] command-line app.  
The script will prompt the user to automatically install `sprongen`,  
if it is not installed.  
`cargo` is required to install `sprongen`.

## Entity Config
Enemies and most entities that can be placed in a level use an _entity config_.  
The entity config is a generic config for adding _components_ to entities and  
for programming entity behavior, based on a reactive event/action system.  
See the [entity-config](./entity-config/README.md) documentation for details.

[sprongen]: https://github.com/Noah2610/sprongen
