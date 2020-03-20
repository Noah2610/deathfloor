use super::system_prelude::*;

#[derive(Default)]
pub struct DeleteBulletsSystem;

impl<'a> System<'a> for DeleteBulletsSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
    }
}
