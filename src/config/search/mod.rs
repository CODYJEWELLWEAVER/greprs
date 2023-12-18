// ENUMS REPRESENTING OPTIONS 
use std::env;
use crate::consts;
use crate::consts::CASE_INSENSITIVE_ARG_0;
use crate::consts::CASE_INSENSITIVE_ARG_1;
use crate::consts::CASE_SENSITIVE_ARG_0;

use super::SearchArgs;
use super::OptionArgs;
// Options for configuring searches

mod test;

#[derive(Debug)]
pub struct SearchConfig<'a> {
    pub query: &'a str,
    pub content: &'a str,
    pub case_sensitive: bool,
}

impl SearchConfig<'_> {
    pub fn new<'a>(
        search_args: SearchArgs<'a>, 
        option_args: OptionArgs
    ) -> Result<SearchConfig<'a>, &'static str> {
        let query = search_args.query;
        let content = search_args.content;

        // Parse Search Options
        let case_sensitive = parse_case_sensitive(option_args);
        
        Ok(
            SearchConfig {
                query,
                content,
                case_sensitive,
            }
        )
    }
}

/**
 * Checks both enviroment variables and cl arguments to determine 
 * if search is case sensitive. CL arguments take priority over 
 * enviroment variables. Returns true if search is case sensitive.
 */
fn parse_case_sensitive(option_args: OptionArgs) -> bool{
    // Check env var.
    let var_result = match env::var_os(consts::CASE_INSENSITIVE_VAR) {
        Some(s) => s == "0",
        None => true,
    };

    // Check for command line argument.
    if option_args.options.len() < 1 {
        // Default to eviroment var if 
        // no args passed in.
        return var_result
    } else {
        let options = option_args.options;
        for option in options {
            match option {
                CASE_INSENSITIVE_ARG_0 |
                CASE_INSENSITIVE_ARG_1 => return false,
                CASE_SENSITIVE_ARG_0 => return true,
                _ => {},
            }
        }

        return true
    };
}