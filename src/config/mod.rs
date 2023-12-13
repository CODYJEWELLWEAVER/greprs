use super::consts;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // Constructor for command line args.
    // Parses cl args and queries enviroment variables.
    // Param: cl args: &[Strings]
    // Return: Result<config: Config, errMsg: &'static str>
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // Requires file and query arguments.
            return Err(
                "Not enough arguments."
            );
        }

        let query: &str = &args[1].clone();
        let filename: &str = &args[2].clone();

        // Check if search is case sensitive
        let case_sensitive: bool = parse_case_sensitive(args);

        Ok( 
            Config { 
                query: query.to_string(), 
                filename: filename.to_string(),
                case_sensitive,
            }
        )   
    }
}

/**
 * Checks both enviroment variables and cl arguments to determine 
 * if search is case sensitive. CL arguments take priority over 
 * enviroment variables.
 */
fn parse_case_sensitive(args: &[String]) -> bool {
    // Check env var.
    let var_result = match env::var_os(consts::CASE_INSENSITIVE_VAR) {
        Some(s) => s == "0",
        None => false,
    };

    // Check for command line argument.
    let arg_result = if args.len() < 4 {
        // Default to eviroment var if 
        // no arg passed in.
        var_result
    } else {
        let case_insensitive: &str = &args[3].clone();
        if case_insensitive == "0" {
            // Overrides env var if false passed in as cli arg
            return true;
        } else if case_insensitive == "1" {
            // Override env var if true passed in as cli arg
            return false
        }
        else {
            // If non-sense arg is passed will ignore the value
            // and will default to case insensitive
            false
        }
    };

    var_result || arg_result
}