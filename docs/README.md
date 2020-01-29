# Documentation
- [DESIGN](./DESIGN.md)
- [ROADMAP](./ROADMAP.md)
- [FEATURES](./FEATURES.md)
- [LEVEL-CREATION](./LEVEL-CREATION.md)

# Concept Art
[`concept-art/`](./concept-art)

# Generating RON files for spritesheets
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

[sprongen]: https://github.com/Noah2610/sprongen
