extern crate deathframe;
#[macro_use]
extern crate derive_builder;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate specs_derive;

mod collision_tag;
mod components;
mod helpers;
mod init;
mod input;
mod map_loader;
mod resources;
mod settings;
mod states;
mod systems;

fn main() {
    if let Err(e) = init::init_game() {
        eprintln!("An error occured: {}", e);
        std::process::exit(1);
    }
}
