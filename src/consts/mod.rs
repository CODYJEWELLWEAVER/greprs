// GREPRS VERSION
pub const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub const UNKNOWN_VERSION: &'static str = "UNKNOWN VERSION";

// QUERY FLAG
pub const QUERY_FLAG: &'static str = "-q:";

// OPTION FLAG
pub const OPTION_FLAG: &'static str = "-";

// CASE SENSITIVITY OPTIONS
pub const CASE_INSENSITIVE_OPTION_0: &'static str = "--ignore-case";
pub const CASE_INSENSITIVE_OPTION_1: &'static str = "-i";

// INFORMATION OPTIONS
pub const GREPRS_HELP_OPTION_0: &'static str = "help";
pub const GREPRS_HELP_OPTION_1: &'static str = "--help";
pub const GREPRS_VERSION_OPTION_0: &'static str = "version";
pub const GREPRS_VERSION_OPTION_1: &'static str = "--version";

// INVERT MATCH OPTIONS
pub const INVERT_MATCH_OPTION_0: &'static str = "-v";
pub const INVERT_MATCH_OPTION_1: &'static str = "--invert-match";

// WORD MATCH OPTIONS
pub const WORD_MATCH_OPTION_0: &'static str = "-w";
pub const WORD_MATCH_OPTION_1: &'static str = "--word-match";

// COUNT OUTPUT LINE OPTIONS
pub const COUNT_OUTPUT_OPTION_0: &'static str = "-c";
pub const COUNT_OUTPUT_OPTION_1: &'static str = "--count";

// GREPRS ERROR MSGS
pub const ERR_MSG_USAGE_HINT: &'static str = "Not enough arguments: Run 'greprs help' for usage help...";
pub const ERR_MSG_PARSING_ARGS: &'static str = "Problem parsing arguments:";
pub const ERR_MSG_NO_QUERIES: &'static str = "No queries in arguments!";
pub const ERR_MSG_NO_FILES: &'static str = "No files in arguments!";
pub const ERR_MSG_APP_ERR: &'static str = "Application error:";
pub const ERR_MSG_STD_ERR_WRITE: &'static str = "Could not write to std err!";
pub const ERR_MSG_NO_SEARCH_CFG: &'static str = "No search configuration! Exiting...";
pub const ERR_MSG_NO_OPEN_FILES: &'static str = "Could not open any files!";
pub const ERR_MSG_NO_VALID_PATTERNS: &'static str = "No valid patterns could be generated...";
pub const ERR_MSG_NO_MATCHES: &'static str = "No matches found in any files...";

/**
* HELP OUTPUT STRING - Output of '$greprs help'
*/
pub const HELP_INFORMATION_OUTPUT: &'static str
= "\x1b[1;38;5;2mWelcome to greprs\x1b[38;5;16m:

\x1b[38;5;2mUsage\x1b[38;5;16m:
    Basic: \x1b[38;5;1m$\x1b[38;5;16mgreprs <query> [<path/to/file>,...] [<option>,...]
    Alt:   \x1b[38;5;1m$\x1b[38;5;16mgreprs [-q:<query>,...] [<path/to/file>,...] [<option>,...]

\x1b[38;5;2mOptions\x1b[38;5;16m:
    Ignore Case:  -i  or  --ignore-case
    Invert Match: -v  or  --invert-match
    Word Match:   -w  or  --word-match
    Count Output: -c  or  --count

\x1b[38;5;2mHelp\x1b[38;5;16m:
    Documentation: \x1b[38;5;1m$\x1b[38;5;16mgreprs help    or  \x1b[38;5;1m$\x1b[38;5;16mgreprs --help
    Version Info: \x1b[38;5;1m$\x1b[38;5;16mgreprs version  or  \x1b[38;5;1m$\x1b[38;5;16mgreprs --version";