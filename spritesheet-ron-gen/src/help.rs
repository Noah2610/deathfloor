pub fn print_help() {
    use crate::meta;

    let help_txt = format!(
        r#"{NAME} v{VERSION}
{DESCRIPTION}

USAGE
    {EXECUTABLE} [OPTIONS] <FILES...>
    {EXECUTABLE} --help

ARGUMENTS
    FILES
        List of file paths to PNG images,
        for which to generate RON files.

OPTIONS
    --help, -h
        Print this help text and exit."#,
        NAME = meta::NAME,
        VERSION = meta::VERSION,
        DESCRIPTION = meta::DESCRIPTION,
        EXECUTABLE = meta::NAME,
    );

    println!("{}", help_txt);

    std::process::exit(0);
}
