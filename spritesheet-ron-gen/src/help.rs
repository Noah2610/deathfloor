pub fn print_help() {
    use crate::meta;
    use crate::size::Size;

    let default_tile_size: String =
        Size::from(crate::ron_generator::DEFAULT_TILE_SIZE).into();

    let help_txt = format!(
        r#"{NAME} v{VERSION}
{DESCRIPTION}

USAGE
    {EXECUTABLE} [OPTIONS] [FLAGS] <FILES...>
    {EXECUTABLE} --help

ARGUMENTS
    FILES
        List of file paths to PNG images,
        for which to generate RON files.

OPTIONS
    --tile-size, -s <SIZE>
        Use the given tile size.
        SIZE format is `<width>x<height>`,
        where <width> and <height> are positive integers.
        DEFAULT: {default_tile_size}

FLAGS
    --verbose, -v
        Enable verbose logging.
        Prints information about used options,
        what PNG files are read,
        and what RON files are being generated.
        Is printed to stderr.

    --pretty, -p
        Pretty format the generated RON files,
        when this option is set.
        Without this, generated RONs will have no new-lines/spacing.

    --help, -h
        Print this help text and exit."#,
        NAME = meta::NAME,
        VERSION = meta::VERSION,
        DESCRIPTION = meta::DESCRIPTION,
        EXECUTABLE = meta::NAME,
        default_tile_size = default_tile_size,
    );

    println!("{}", help_txt);

    std::process::exit(0);
}
