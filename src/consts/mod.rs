// Name of case sensitivity variable. Set with export CASE_INSENSITIVE=(0 or 1)
pub const CASE_INSENSITIVE_VAR: &str = "CASE_INSENSITIVE";

// GREPRS VERSION
pub const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub const UNKNOWN_VERSION: &str = "UNKNOWN VERSION";

// CASE SENSITIVITY OPTIONS
pub const CASE_INSENSITIVE_OPTION_0: &str = "-ignore-case";
pub const CASE_INSENSITIVE_OPTION_1: &str = "-i";
pub const CASE_SENSITIVE_OPTION_0: &str = "-no-ignore-case";

// INFORMATION OPTIONS
pub const GREPRS_HELP_OPTION: &str = "help";
pub const GREPRS_VERSION_OPTION: &str = "version";

// INVERT MATCH OPTIONS
pub const INVERT_MATCH_OPTION_0: &str = "-v";
pub const INVERT_MATCH_OPTION_1: &str = "-invert-match";

// GREPRS ERROR MSGS
pub const ERROR_MSG_USAGE_HINT: &'static str = "Not enough arguments: Run 'greprs help' for usage help...";

// HELP INFORMATION OUTPUT
///////////////////////////
// Output of '$greprs help'
pub const HELP_INFORMATION_OUTPUT: &str
= "Welcome to greprs:
*******************************************************************************
Usage: 
  -Basic: $greprs <query> <path/to/file>
  -Basic w/ options: $greprs <query> <path/to/file> [<option>, ...]
  -Alt: $greprs <query> in <path/to/file>
  -Alt w/ options: $greprs <query> in <path/to/file> with [<option>, ...]
*******************************************************************************
Help:
  -Documentation: $greprs help
  -Version Info: $greprs version";