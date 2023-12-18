// ENUMS REPRESENTING OPTIONS 
use std::env;
use crate::consts;

use super::SearchArgs;
use super::OptionArgs;
// Options for configuring searches
pub struct SearchConfig<'a> {
    pub query: &'a str,
    pub content: &'a str,
    pub case_sensitive: bool,
}

impl SearchConfig<'_> {
    pub fn new<'a>(search_args: SearchArgs<'a>, option_args: OptionArgs) -> SearchConfig<'a> {
        let query = search_args.query;
        let content = search_args.content;
        let case_sensitive = Self::parse_case_sensitive(option_args.options);
        
        SearchConfig {
            query,
            content,
            case_sensitive,
        }
    }

    /**
     * Checks both enviroment variables and cl arguments to determine 
     * if search is case sensitive. CL arguments take priority over 
     * enviroment variables.
     */

    fn parse_case_sensitive(option_args: Vec<&String>) -> bool {
        // Check env var.
        let var_result = match env::var_os(consts::CASE_INSENSITIVE_VAR) {
            Some(s) => s == "0",
            None => false,
        };
    
        // Check for command line argument.
        let arg_result = if option_args.len() < 4 {
            // Default to eviroment var if 
            // no arg passed in.
            var_result
        } else {
            let case_insensitive: &str = &option_args[3].clone();
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
}



