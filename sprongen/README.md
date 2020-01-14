# `sprongen`
`Spr`itesheet  
`RON`  
`Gen`erator

## Description
Generates spritesheet config RON files for your spritesheet PNG images.
The spritesheet configs are generated for the `amethyst` engine.
Generated RON files are placed next to the corresponding PNG image file.

## Installation
After cloning the parent `deathfloor` repo,  
you can install it locally using `cargo` with ...
```
cargo install --force --path ./deathfloor/sprongen
```

## Usage
```
sprongen --help
```

```
sprongen 0.0.0
Generate RON files for the given spritesheet PNG images

USAGE:
    sprongen [FLAGS] [OPTIONS] <FILES>...

FLAGS:
    -h, --help
            Prints help information

    -p, --pretty
            Pretty format the generated RON files.

            Without this, generated RON files will have no new-lines/spacing.
    -V, --version
            Prints version information

    -v, --verbose
            Enable verbose logging.

            Prints information about used options, what PNG files are read, and what RON files are being generated. Is
            printed to stderr.

OPTIONS:
    -s, --tile-size <tile-size>
            Use the given tile size.

            <tile-size> format is `<width>x<height>`, where <width> and <height> are positive integers. [default: 32x32]

ARGS:
    <FILES>...

```
