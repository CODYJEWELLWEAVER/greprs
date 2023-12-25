// GREPRS VERSION
pub const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub const UNKNOWN_VERSION: &'static str = "UNKNOWN VERSION";

// CASE SENSITIVITY OPTIONS
pub const CASE_INSENSITIVE_OPTION_0: &'static str = "-ignore-case";
pub const CASE_INSENSITIVE_OPTION_1: &'static str = "-i";
pub const CASE_SENSITIVE_OPTION_0: &'static str = "-no-ignore-case";

// INFORMATION OPTIONS
pub const GREPRS_HELP_OPTION: &'static str = "help";
pub const GREPRS_VERSION_OPTION: &'static str = "version";

// INVERT MATCH OPTIONS
pub const INVERT_MATCH_OPTION_0: &'static str = "-v";
pub const INVERT_MATCH_OPTION_1: &'static str = "-invert-match";

// COUNT OUTPUT LINE OPTIONS
pub const COUNT_OUTPUT_OPTION_0: &'static str = "-c";
pub const COUNT_OUTPUT_OPTION_1: &'static str = "-count";

// GREPRS ERROR MSGS
pub const ERR_MSG_USAGE_HINT: &'static str = "Not enough arguments: Run 'greprs help' for usage help...";
pub const ERR_MSG_PARSING_ARGS: &'static str = "Problem parsing arguments:";
pub const ERR_MSG_APP_ERR: &'static str = "Application error:";
pub const ERR_MSG_STD_ERR_WRITE: &'static str = "Could not write to std err!";

// HELP INFORMATION OUTPUT
///////////////////////////
// Output of '$greprs help'
pub const HELP_INFORMATION_OUTPUT: &'static str
= "Welcome to greprs:
*******************************************************************************
Usage: 
  -Basic: $greprs <query> <path/to/file>
  -Basic w/ options: $greprs <query> [<path/to/file>, ...] [<option>, ...]
  -Alt: $greprs <query> in <path/to/file>
  -Alt w/ options: $greprs <query> in [<path/to/file>, ...] [<option>, ...]
*******************************************************************************
Help:
  -Documentation: $greprs help
  -Version Info: $greprs version";