use crate::opts::Opts;
use crate::ron_generator::GenerateOptions;
use std::path::PathBuf;
use structopt::StructOpt;

pub enum Action {
    Gen(Vec<PathBuf>, GenerateOptions),
    Help,
}

impl Action {
    pub fn new() -> Result<Self, String> {
        let opts = Opts::from_args();
        Ok(if opts.files.is_empty() {
            Self::default()
        } else {
            let generate_options = GenerateOptions {
                verbose:   opts.verbose,
                pretty:    opts.pretty,
                tile_size: opts.tile_size,
            };
            Action::Gen(opts.files, generate_options)
        })
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::Help
    }
}
