extern crate deathframe;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate specs_derive;

mod components;
mod helpers;
mod init;
mod input;
mod solid_tag;
mod states;
mod systems;

fn main() {
    if let Err(e) = init::init_game() {
        eprintln!("An error occured: {}", e);
        std::process::exit(1);
    }
}
