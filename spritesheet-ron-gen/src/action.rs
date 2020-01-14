use crate::ron_generator::GenerateOptions;
use crate::size::Size;
use std::convert::TryFrom;
use std::env;
use std::path::PathBuf;

pub enum Action {
    Gen(Vec<PathBuf>, GenerateOptions),
    Help,
}

impl Action {
    pub fn current() -> Result<Self, String> {
        const HELP_OPTS: [&str; 3] = ["help", "--help", "-h"];
        const TILE_SIZE_OPTS: [&str; 2] = ["--tile-size", "-s"];

        let mut generate_options = GenerateOptions::default();
        let mut files = Vec::new();

        let mut next_arg_is_tile_size = false;

        for arg in env::args().skip(1) {
            let mut add_arg_as_file = true;

            let s = arg.as_str();
            if HELP_OPTS.contains(&s) {
                return Ok(Action::Help);
            }
            if next_arg_is_tile_size {
                next_arg_is_tile_size = false;
                add_arg_as_file = false;
                generate_options.tile_size = Size::try_from(s)?;
            } else if TILE_SIZE_OPTS.contains(&s) {
                add_arg_as_file = false;
                next_arg_is_tile_size = true;
            }

            if add_arg_as_file {
                let file = PathBuf::from(s);
                files.push(file);
            }
        }

        Ok(if files.is_empty() {
            Self::default()
        } else {
            Action::Gen(files, generate_options)
        })
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::Help
    }
}
