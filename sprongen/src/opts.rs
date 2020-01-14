use crate::meta;
use crate::size::Size;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Clone, StructOpt, Debug)]
#[structopt(
    name = meta::NAME,
    version = meta::VERSION,
    about = meta::DESCRIPTION,
)]
pub struct Opts {
    /// Enable verbose logging.
    ///
    /// Prints information about used options,
    /// what PNG files are read,
    /// and what RON files are being generated.
    /// Is printed to stderr.
    #[structopt(short, long)]
    pub verbose: bool,

    /// Pretty format the generated RON files.
    ///
    /// Without this, generated RON files will have no new-lines/spacing.
    #[structopt(short, long)]
    pub pretty: bool,

    /// Use the given tile size.
    ///
    /// <tile-size> format is `<width>x<height>`,
    /// where <width> and <height> are positive integers.
    #[structopt(short = "s", long, default_value = "32x32")]
    pub tile_size: Size,

    #[structopt(name = "FILES", multiple = true, required = true)]
    pub files: Vec<PathBuf>,
}
