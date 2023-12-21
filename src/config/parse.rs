use std::collections::HashMap;
use crate::config::OptionArgs;
use crate::config::SearchArgs;
use crate::consts::COUNT_OUTPUT_OPTION_0;
use crate::consts::COUNT_OUTPUT_OPTION_1;
use crate::consts::{ CASE_INSENSITIVE_OPTION_0, INVERT_MATCH_OPTION_0, INVERT_MATCH_OPTION_1,
CASE_INSENSITIVE_OPTION_1 };

use super::OptionType;


#[cfg(test)]
mod test {
    use crate::config::OptionType;
    use super::*;

    #[test]
    fn parse_options() {
        let test_args = ["-i".to_string(),];
        let option_args = parse_option_args(&test_args).unwrap();
        let mut map: HashMap<OptionType, Vec<& str>> = HashMap::new();
        map.insert(OptionType::CaseInsensitive, Vec::new());
        assert_eq!(option_args.options, map);
    }
}

// Parses input args and maps option markups with
// their respective values. Option args are then passed into
// SearchConfig, OutputConfig, etc..
pub fn parse_option_args<'a>(args: &'a[String]) -> Result<OptionArgs, &'static str> {
    let mut options: HashMap<OptionType, Vec<&'a str>> = HashMap::new();
    
    for i in 0..args.len() {
        // Parse options that take some sort of input for configuration.
        if args[i].starts_with("--") {
            if i == args.len() - 1 {
                return Err("Cannot parse options, run 'greprs help' for usage help.")
            }
            let mut option_values: Vec<& str> = Vec::new();
            option_values.push(&args[i+1]);
            // TODO: ADD OPTIONS THAT REQUIRE INPUT
            
        }
        // Parse options that need no additional input for configuration.
        else if args[i].starts_with("-") {
            let option_type = match_option_type(&args[i]);
            match option_type {
                OptionType::Unknown => {
                    return Err("Unknown option parameter!")
                }
                _ => {
                    // For options that need no 
                    // values passed in adds 
                    // and empty vector to map.
                    options.insert(option_type, Vec::new());
                }
            };
        }
    }

    Ok(OptionArgs{ options })
}

fn match_option_type(arg: & str) -> OptionType {
    return match arg {
        // Simple options (no additional values needed)
        // Case insensitive options
        CASE_INSENSITIVE_OPTION_0 |
        CASE_INSENSITIVE_OPTION_1 => OptionType::CaseInsensitive,
        // Invert match Options
        INVERT_MATCH_OPTION_0 |
        INVERT_MATCH_OPTION_1 => OptionType::InvertMatch,
        // Count output options
        COUNT_OUTPUT_OPTION_0 |
        COUNT_OUTPUT_OPTION_1 => OptionType::CountOutput,
        // Value options (additional values needed)
        _ => OptionType::Unknown,
    }
}

// Parses input args for query and content arguments
// Returns Err(msg) on failure and SearchArgs on success.
pub fn parse_search_args<'a>(args: &'a[String]) -> Result<SearchArgs, &'static str> {
    if args[2] != "in".to_string() {
        let query: &str = &args[1];
        let content: &str = &args[2]; 

        if query.is_empty() || content.is_empty() {
            return Err(
                "Error parsing query and content args. Run 'greprs help' for detailed information."
            )
        }
        else {
            return Ok(SearchArgs{query, content})
        }
    }
    else {
        // Define query arguments using <query> in <content> syntax.
        // Default for content is a filename.
        let query: &str = &args[1];
        let content: &str = &args[3];
        Ok(SearchArgs{query, content})
    }
}
/* 
/**
 * Checks both enviroment variables and cl arguments to determine 
 * if search is case sensitive. CL arguments take priority over 
 * enviroment variables. Returns true if search is case sensitive.
 */
pub fn parse_case_sensitive(option_args: &OptionArgs) -> bool{
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
        let options = &option_args.options;
        for option in options {
            match *option {
                CASE_INSENSITIVE_OPTION_0 |
                CASE_INSENSITIVE_OPTION_1 => return false,
                CASE_SENSITIVE_OPTION_0 => return true,
                _ => {},
            }
        }

        return true
    };
}

/*
 * Parses options list to find an 
 * invert match option.
 */
pub fn parse_invert_match(option_args: &OptionArgs) -> bool {
    let options: &Vec<& str> = &option_args.options;
    if !options.is_empty() {
        for option in options {
            match *option {
                INVERT_MATCH_OPTION_0 |
                INVERT_MATCH_OPTION_1 => {
                    return true
                },
                _ => {}
            }
        }

        return false
    }

    return false
}

/*
 * Parses options list to find a
 * count output lines option.
*/
pub fn parse_count_option(option_args: &OptionArgs) -> bool {
    let options: &Vec<& str> = &option_args.options;
    if !options.is_empty() {
        for option in options {
            match *option {
                COUNT_OUTPUT_OPTION_0 |
                COUNT_OUTPUT_OPTION_1 => {
                    return true
                }
                _ => {}
            }
        }
    }

    return false
} */