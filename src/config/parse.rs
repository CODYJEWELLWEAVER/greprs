use std::collections::HashMap;
use std::ops::{RangeBounds, Bound};
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

/**
Parses option arguments and additional values that may be passed in 
to options.
*/
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

/**
 Returns a OptionType that corresponds to a given option 
 argument string, or OptionType::Unknown for invalid option args.
 */
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
        _ => OptionType::Unknown,
    }
}

/**
Parses input args for query and content arguments
* returns: Err(msg) on failure and SearchArgs on success.
*/
pub fn parse_search_args<'a>(args: &'a[String]) -> Result<SearchArgs, &'static str> {
    
    let mut queries: Vec<&str> = Vec::new();
    let mut files: Vec<&str> = Vec::new();
    
    // Check first argument for an option
    // Otherwise add it to queries
    if !args[1].starts_with("-") {
        // remove query indicator
        if args[1].starts_with("q:") && args[1].len() > 2{
            queries.push(&args[1].slice(2..));
        } else {
            queries.push(&args[1]);
        }
    }
    
    // Check for queries & files in args
    for arg in &args[2..] {
        if !arg.starts_with('-') {
            if arg.starts_with("q:") && arg.len() > 2 {
                queries.push(&arg.slice(2..));
            }
            else {
                files.push(arg);
            }
        }
    }

    return if queries.is_empty() || files.is_empty() {
        Err(
            "Error parsing query and file args. Run 'greprs help' for detailed information."
        )
    } else {
        Ok(SearchArgs{queries, files})
    }
}

/**
 * String slice utitily function. 
 * Credit to carlomilanesi: https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/11
 */
trait StringUtils {
    fn substring(&self, start: usize, length: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, length: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == length { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}