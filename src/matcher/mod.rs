use std::error;
use regex::Regex;
use crate::{SearchConfig, consts};


mod test;

/**
  Groups one or more pattern together as well as 
  options for matching with these patterns
  * patterns: vector of patterns to match against
  * invert: option to invert match results
 */
pub struct MatchPattern {
    pub patterns: Vec<Regex>,
    pub invert_match: bool
}

impl MatchPattern {
    /** MatchPattern Constructor */
    pub fn new(search_config: &SearchConfig) 
                -> Result<MatchPattern, Box<dyn error::Error>> {
        let mut query_string: String = String::new();
        let patterns: Vec<Regex> 
            = search_config.queries.iter()
                .filter_map( |query| 
                    {
                        query_string = if search_config.word_match {
                            format!(r"\b{}\b", query)
                        } else {
                            format!(r"{}", query)
                        };
                        if !search_config.case_sensitive {
                            query_string = format!(r"(?i){}", query_string)
                        }

                        Regex::new(&query_string).ok()
                    }
                )
                .collect::<Vec<Regex>>();

        if patterns.is_empty() {
            Err(
                Box::from(consts::ERR_MSG_NO_VALID_PATTERNS)
            )
        } else {
            Ok(
                MatchPattern { 
                    patterns, 
                    invert_match: search_config.invert_match
                }
            )
        }
    }

    /** Checks if a given line matches with any pattern */
    pub fn matches(&self, line: &str) -> bool {
        return self.patterns.iter()
            .filter(|pattern| (pattern.is_match(line) != self.invert_match))
            .collect::<Vec<&Regex>>()
            .len() != 0usize;
   }
}