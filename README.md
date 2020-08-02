# deathfloor
__Work in progress...__  
__Working title...__

## Description
A little platformer game, in which the player must  
jump their way upwards to escape the _deathfloor_.

## Running
Binaries are not yet available.

To compile and run from source, first `clone` the repo,  
then compile and run the project with `cargo` ...
```
cargo run --release
```
Or you can use the development script `bin/run`.  
Note that certain dependencies are required for this script,  
such as `rustup` and a specific `nightly` rust toolchain;  
run the script to see what's missing.  
Any arguments are passed to the underlying `cargo run` command.

## Documentation
See [`docs`][docs].

## Licensing
### Code
All code for this project is distributed under the terms of the [MIT License][license-mit].

### Assets
Assets, including all image and audio files under the [`resources/`][resources] directory,  
and all concept art under the [`docs/concept-art/`][concept-art] directory of this project,  
excluding fonts, are distributed under the terms of the  
[Creative Commons Attribution-NonCommercial 4.0 International License][license-cc].

### Fonts
Used fonts are distributed under their respective licenses:
- [undefined medium][font-undefined-medium]

[docs]:                  ./docs
[resources]:             ./resources
[concept-art]:           ./docs/concept-art
[license-mit]:           ./LICENSE
[license-cc]:            https://creativecommons.org/licenses/by-nc/4.0/
[font-undefined-medium]: https://github.com/andirueckel/undefined-medium
