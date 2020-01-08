extern crate deathframe;
extern crate ron;

mod components;
mod helpers;
mod init;
mod states;
mod systems;

fn main() {
    if let Err(e) = init::init_game() {
        eprintln!("An error occured: {}", e);
        std::process::exit(1);
    }
}
