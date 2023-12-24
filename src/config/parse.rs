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
    let mut files: Vec<&str> = Vec::new();
    if args[2] != "in".to_string() {
        let query: &str = &args[1];
        // Check for files in &args
        for arg in &args[2..] {
            if !arg.starts_with('-') {
                files.push(arg);
            }
        }

        if query.is_empty() || files.is_empty() {
            return Err(
                "Error parsing query and content args. Run 'greprs help' for detailed information."
            )
        } else {
            return Ok(SearchArgs{query, files})
        }
    }
    else {
        // Define query arguments using <query> in [<files>, ...] syntax.
        // Default for content is a filename.
        let query: &str = &args[1];
        for arg in &args[3..] {
            if !arg.starts_with('-') {
                files.push(arg);
            }
        }
        
        return if files.is_empty() {
            Err("No file arguments!")
        } else {
            Ok(SearchArgs{query, files})
        }
    }
}