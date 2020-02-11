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

// TODO
mod get_by_axis {
    use deathframe::core::geo::prelude::Axis;

    pub trait GetByAxis {
        type Item;
        fn by_axis(self, axis: &Axis) -> Self::Item;
    }

    impl<'a, T> GetByAxis for &'a (T, T) {
        type Item = &'a T;
        fn by_axis(self, axis: &Axis) -> Self::Item {
            match axis {
                Axis::X => &self.0,
                Axis::Y => &self.1,
            }
        }
    }

    impl<'a, T> GetByAxis for &'a mut (T, T) {
        type Item = &'a mut T;
        fn by_axis(self, axis: &Axis) -> Self::Item {
            match axis {
                Axis::X => &mut self.0,
                Axis::Y => &mut self.1,
            }
        }
    }
}
