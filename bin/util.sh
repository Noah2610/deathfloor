## # util.sh
## Version: `2.2.5`
## https://github.com/Noah2610/util.sh

set -o pipefail

## Returns `0` or `1` depending on if the given string is available as a command.
function is_available {
    local cmd="$1"
    [ -z "$cmd" ] && err "No command to check for availability given to function \`is_available\`"
    command -v "$cmd" &> /dev/null
}

## Returns `0` or `1` depending on if the command is running (using `pgrep`).
function is_running {
    check "pgrep"
    local to_check="$1"
    [ -z "$to_check" ] && err "No command to check if running given to function \`is_running\`"
    pgrep -xc "$to_check" &> /dev/null
}

## Exit with error message if the given string is not available as a command.
function check {
    local cmd="$1"
    [ -z "$cmd" ] && err "No command to check given to function \`check\`"
    is_available "$cmd" &> /dev/null \
        || err "\`$(clr "${CLR_CODE[@]}")${cmd}$(clrrs)\` is not available."
}

## Exit with error message if the given file path does not exist.
function check_file {
    local FILE="$1"
    [ -z "$FILE" ] && err "No file given to function \`check_file\`"
    [ -f "$FILE" ] || err "File does not exist: $(clr "${CLR_CODE[@]}")${FILE}$(clrrs)"
}

## Exit with error message if the given directory path does not exist.
function check_dir {
    local DIR="$1"
    [ -z "$DIR" ] && err "No directory given to function \`check_dir\`"
    [ -d "$DIR" ] || err "Directory does not exist: $(clr "${CLR_CODE[@]}")${DIR}$(clrrs)"
}

## Print the given string to stdout.
function msg {
    print_log "$1"
    echo -e "$(clr "${CLR_MSG[@]}")${1}$(clrrs)"
}

## Print the given string to stderr and to the `$LOGFILE` (if one exists),
## then exit with exit code 1.
function err {
    print_log "$( semantic_date )\nERROR: $1"
    (1>&2 echo -e "$(clr "${CLR_ERR[@]}")ERROR:$(clrrs) ${1}\nExiting.")
    exit 1
}

## Print the given string to stderr and to the `$LOGFILE`
function warn {
    print_log "WARNING: $1"
    (1>&2 echo -e "$(clr "${CLR_WARN[@]}")WARNING:$(clrrs) ${1}")
}

## Print the given string to the `$LOGFILE` (if one exists), and strip color from the text.
function print_log {
    local txt="$1"
    [ -z "$txt" ] && err "No message text given to function \`print_log\`"
    [ -n "$LOGFILE" ] && echo -e "$( _strip_ansi_codes "$txt" )\n" >> "$LOGFILE"
}

# Strip ansi codes from the given string.
function _strip_ansi_codes {
    local txt="$1"
    if is_available "sed"; then
        echo -e "$txt" | sed "s,\x1B\[[0-9;]*[a-zA-Z],,g"
    else
        echo -e "$txt"
    fi
}

# https://misc.flogisoft.com/bash/tip_colors_and_formatting
## Echos the _FOREGROUND_ color code matching the given color argument.
## The given color string is a semantic representation of color.
## Available colors:
## ```
##   "black", "white", "red", "green", "blue",
##   "yellow", "magenta", "cyan", "lightgray", "darkgray",
##   "lightred", "lightgreen", "lightblue", "lightyellow",
##   "lightmagenta", "lightcyan",
##   "default", "reset"
## ```
function clrfg {
    [ -n "$NO_COLOR" ] && return 0
    local color_str="$1"
    local color_code
    [ -z "$color_str" ] && err "No color argument given to function \`clrfg\`"
    color_code="$( _color_code_generic "$color_str" )"
    echo -en "\e[${color_code}m"
}

## Echos the _BACKGROUND_ color code matching the given color argument.
## The given color string is a semantic representation of color.
## Available colors:
## ```
##   "black", "white", "red", "green", "blue",
##   "yellow", "magenta", "cyan", "lightgray", "darkgray",
##   "lightred", "lightgreen", "lightblue", "lightyellow",
##   "lightmagenta", "lightcyan",
##   "default", "reset"
## ```
function clrbg {
    [ -n "$NO_COLOR" ] && return 0
    local color_str="$1"
    local color_code
    [ -z "$color_str" ] && err "No color argument given to function \`clrbg\`"
    color_code="$(( 10 + $( _color_code_generic "$color_str" ) ))"
    echo -en "\e[${color_code}m"
}

## Echos the attribute color code matching the given argument.
## The given argument string is a semantic representation of an attribute.
## Available attributes:
## ```
##   "bold", "dim", "underline", "blink", "invert", "hidden",
##   "default", "reset"
## ```
function clrattr {
    [ -n "$NO_COLOR" ] && return 0
    local attr_str="$1"
    [ -z "$attr_str" ] && err "No attribute argument given to function \`clrattr\`"
    case "$attr_str" in
        "bold")      echo -en "\e[1m" ;;
        "dim")       echo -en "\e[2m" ;;
        "underline") echo -en "\e[4m" ;;
        "blink")     echo -en "\e[5m" ;;
        "invert")    echo -en "\e[7m" ;;
        "hidden")    echo -en "\e[8m" ;;
        "default")   echo -en "\e[20m" ;;
        "reset")     echo -en "\e[20m" ;;
        *) err "Invalid attribute argument: \"${attr_str}\"" ;;
    esac
}

## Set the foreground, background, and attribute colors all at once.
## This function takes three arguments:
## - foreground color (see `clrfg`)
## - background color (see `clrbg`)
## - attribute (see `clrattr`)
function clr {
    [ -n "$NO_COLOR" ] && return 0
    local color_fg="$1"
    local color_bg="$2"
    local color_attr="$3"
    if [ -z "$color_fg" ] || [ -z "$color_bg" ] || [ -z "$color_attr" ]; then
        err "Function \`clr\` requires 3 arguments"
    fi
    clrfg "$color_fg"
    clrbg "$color_bg"
    clrattr "$color_attr"
}

## Resets all color settings to default.
function clrrs {
    [ -n "$NO_COLOR" ] && return 0
    echo -en "\e[0m"
}

# Internal helper function.
# Returns the foreground color code number for the given color string.
# This is then used in the above color functions to create
# proper, printable color codes for foreground and background.
function _color_code_generic {
    local color_str="$1"
    [ -z "$color_str" ] && err "No color argument given to function \`_color_code_generic\`"
    case "$color_str" in
        "black")        echo -n "30" ;;
        "white")        echo -n "97" ;;
        "red")          echo -n "31" ;;
        "green")        echo -n "32" ;;
        "blue")         echo -n "34" ;;
        "yellow")       echo -n "33" ;;
        "magenta")      echo -n "35" ;;
        "cyan")         echo -n "36" ;;
        "lightgray")    echo -n "37" ;;
        "darkgray")     echo -n "90" ;;
        "lightred")     echo -n "91" ;;
        "lightgreen")   echo -n "92" ;;
        "lightblue")    echo -n "94" ;;
        "lightyellow")  echo -n "93" ;;
        "lightmagenta") echo -n "95" ;;
        "lightcyan")    echo -n "96" ;;
        "default")      echo -n "39" ;;
        "reset")        echo -n "39" ;;
        *) err "Invalid color argument: \"${color_str}\"" ;;
    esac
}

# __TODO:__ Refactor?
## Print out a date string in a specifc format.
## If the command `boxed-string` is available, then it calls that with the date string.
## boxed-string: https://gist.github.com/Noah2610/2c4a92f6732419becade2f76bc943039
function semantic_date {
    check "date"
    local dfmt
    local dstr
    dfmt='+%F %T'
    dstr="$( date "$dfmt" )"
    if is_available "boxed-string"; then
        BOXED_PADDING_HORZ=1 \
        BOXED_PADDING_VERT=0 \
            boxed-string -- "$dstr"
    else
        echo "$dstr" | tee -a "$LOGFILE"
    fi
}

## Tries to run the given command.
## All arguments are parsed as the command, so the first argument is the
## command, and all following arguments are passed as arguments to the command.
## Runs `err` if the command fails.
## Writes the command's output to the `$LOGFILE`.
function try_run {
    local cmd=( "$@" )
    [ ${#cmd} -lt 1 ] && err "No command given to function \`try_run\`"
    local out
    local cmd_display
    # TODO: This doesn't 100% correctly display the executed command,
    #       because the command is an array, and each element is its own
    #       argument, but this string doesn't reflect that.
    cmd_display="$( clr "${CLR_CODE[@]}")${cmd[*]}$(clrrs )"

    # shellcheck disable=SC2154
    msg "${spacing}Running: ${cmd_display}"
    if ! "${cmd[@]}" | tee -a "$LOGFILE"; then
        err "Command failed: ${cmd_display}"
    fi
}

## Similar to function `try_run_hidden`, but hides the command's output.
## Tries to run the given command and hides its output.
## All arguments are parsed as the command, so the first argument is the
## command, and all following arguments are passed as arguments to the command.
## If the command fails, then it prints the output with `err`.
## Writes the command's output to the `$LOGFILE`.
function try_run_hidden {
    local cmd=( "$@" )
    [ ${#cmd} -lt 1 ] && err "No command given to function \`try_run_hidden\`"
    local out
    local cmd_display
    # TODO: See TODO note in `try_run`
    cmd_display="$( clr "${CLR_CODE[@]}")${cmd[*]}$(clrrs )"

    # shellcheck disable=SC2154
    msg "${spacing}Running: ${cmd_display}"
    if ! out="$( "${cmd[@]}" 2>&1 | tee -a "$LOGFILE" )"; then
        err "\
Command failed: ${cmd_display}
Returned:
${out}"
    fi
}

## Returns `0` if the given argument represents a "positive" value (not empty and non-0).
function is_positive {
    [ -n "$1" ] && [ "$1" != "0" ]
}

## Returns `0` if the given argument represents a "negative" value (empty or 0).
function is_negative {
    [ -z "$1" ] || [ "$1" = "0" ]
}

## Returns `0` or `1` depending on if the final command should be run in a new terminal.
## For very specific use-case(s).
function should_run_in_terminal {
    is_positive "$RUN_TERMINAL" \
        && [ -n "$TERMINAL" ] \
        && is_available "$TERMINAL"
}

## Run the given command in a new terminal.
## The first argument is the command, any following arguments
## are passed to the command as its arguments.
## The new shell's working directory is set to the `$ROOT` variable.
function run_terminal {
    local cmd=( "$@" )
    [ ${#cmd} -gt 0 ] || err "No command given to function \`run_terminal\`."
    local cmd_bash="bash -c '${cmd[*]} || (echo -e \"\n[CONTINUE]\"; read)'"

    check "$TERMINAL"
    case "$TERMINAL" in
        "termite")
            termite -d "$ROOT" -e "$cmd_bash" & \
            disown
            ;;
        *)
            err "Function \`run_terminal\` is not configured for terminal '${TERMINAL}'"
            ;;
    esac
}

## Returns `0` or `1` depending on if the user answers
## positively (`y`) or negatively (`n`).
## The first argument is the message/question printed to `stdout`.
function prompt_question {
    local msg="$1 [y|n] "
    local input=
    while [ -z "$input" ]; do
        echo -en "$msg"
        read -rn1 input
        echo
        case "${input:0:1}" in
            "y"|"Y") return 0 ;;
            "n"|"N") return 1 ;;
            *) input= ;;
        esac
    done
}

## Returns `0` if the given argument is an absolute path (starts with "/").
function is_absolute_path {
    [[ "$1" = /* ]]
}

## Returns `0` if the given argument is a relative path (does _not_ start with "/").
function is_relative_path {
    [[ "$1" != /* ]]
}

## https://stackoverflow.com/a/17841619
function join_by {
    local d=$1
    shift
    echo -n "$1"
    shift
    printf "%s" "${@/#/$d}"
}

## `realpath` wrapper function.
## Executes `realpath` if it is available, otherwise
## it just uses a combination of `cd` and `pwd`.
function realpath {
    if is_available "realpath"; then
        command realpath "$@"
    else
        pushd "$( dirname "$1" )" &> /dev/null || exit 1
        echo "$( pwd )/$( basename "$1" )"
        popd &> /dev/null || exit 1
    fi
}

# Initialize the helper script.
# Is run at the end of this script.
function _init {
    check "basename"
    check "dirname"
    check "tee"

    local script_name="$1"
    # shellcheck disable=SC2155
    local script_path="$( realpath "$script_name" )"

    # Set `$ROOT` variable to the directory of this script,
    # unless it was already set.
    # If the name of the directory is 'bin', then set `$ROOT`
    # to the parent directory of 'bin/'.
    [ -z "$ROOT" ] \
        && ROOT="$( cd "$( dirname "$script_path" )" || exit 1; pwd )" \
        && [ "$( basename "$ROOT" )" = "bin" ] \
        && ROOT="$( dirname "$ROOT" )"

    # Set the `$LOGFILE` variable unless it was already set.
    [ -z "$LOGFILE" ] \
        && LOGFILE="${ROOT}/.$( basename "$script_path" ).log"

    # Create the directory path to `$LOGFILE` if it doesn't exist.
    logfile_dir="$( dirname "$LOGFILE" )"
    ! [ -d "$logfile_dir" ] && mkdir -p "$logfile_dir"
    unset logfile_dir

    # Set the `$TERMINAL` variable unless it was already set.
    [ -z "$TERMINAL" ] && TERMINAL="termite"

    # Set `$NO_COLOR` to disable color output.
    [ -z "$NO_COLOR" ] && NO_COLOR=

    # Set some commonly used colors.
    # Arrays have the format of `( "fgcolor" "bgcolor" "attribute" )`.
    CLR_ERR=( "white" "red" "bold" )
    CLR_WARN=( "black" "yellow" "bold" )
    CLR_MSG=( "yellow" "default" "default" )
    CLR_CODE=( "blue" "black" "default" )
}

_init "$0"
