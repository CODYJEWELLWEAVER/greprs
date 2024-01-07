use std::ops::{RangeBounds, Bound};
use crate::config::SearchArgs;
use crate::consts::{COUNT_OUTPUT_OPTION_0, WORD_MATCH_OPTION_0, WORD_MATCH_OPTION_1, self};
use crate::consts::COUNT_OUTPUT_OPTION_1;
use crate::consts::{ CASE_INSENSITIVE_OPTION_0, INVERT_MATCH_OPTION_0, INVERT_MATCH_OPTION_1,
CASE_INSENSITIVE_OPTION_1 };

use super::OptionType;


#[cfg(test)]
mod test {
    use crate::config::OptionType;
    use super::*;

    #[test]
    fn parse_args() {
        let args  = ["/greprs".to_string(), "in".to_string(), "res/test/haiku.txt".to_string(), "-i".to_string(), "-q:hello".to_string(), "--count".to_string()];
        let search_args = parse_arguments(&args).unwrap();
        let expected_queries = vec!("in".to_string(), "hello".to_string());
        let expected_files = vec!("res/test/haiku.txt".to_string());
        let expected_options = vec!(OptionType::CaseInsensitive, OptionType::CountOutput);
        assert_eq!(expected_files, search_args.files);
        assert_eq!(expected_queries, search_args.queries);
        assert_eq!(expected_options, search_args.options);
    }
}

pub fn parse_arguments<'a>(args: &'a[String]) -> Result<SearchArgs, &'static str> {
    let mut queries: Vec<&str> = Vec::new();
    let mut files: Vec<&str> = Vec::new();
    let mut options: Vec<OptionType> = Vec::new();
    
    // parse queries, files, and options
    (&args[1..]).into_iter().for_each(|arg| {
        if arg.starts_with(consts::QUERY_FLAG) && arg.len() > 2 {
            let query_strings: Vec<& str> = arg.slice(2..).split(':').collect();
            query_strings.iter().for_each(|query| {
                if *query != "" {
                    queries.push(query);
                }
            });
        }
        else if !arg.starts_with(consts::OPTION_FLAG) {
            if queries.is_empty() {
                // add first non option arg to query
                queries.push(arg);
            } else {
                files.push(arg);
            }
        } else {
            let option_type: OptionType = match_option_type(arg);
            if option_type != OptionType::Unknown {
                options.push(option_type);
            }
        }
    });

    if queries.is_empty() {
        return Err(consts::ERR_MSG_NO_QUERIES)
    }
    if files.is_empty() {
        return Err(consts::ERR_MSG_NO_FILES)
    }

    Ok(
        SearchArgs::new(queries, files, options)
    )
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
        // Word match options 
        WORD_MATCH_OPTION_0 |
        WORD_MATCH_OPTION_1 => OptionType::WordMatch,
        _ => OptionType::Unknown,
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