use regex::Regex;
use crate::SearchConfig;

mod test;

/**
  Groups one or more pattern together as well as 
  options for matching with these patterns
  * patterns: vector of patterns to match against
  * invert: option to invert match results
 */
pub struct MatchPattern {
    pub patterns: Vec<Regex>,
    pub invert: bool
}

impl MatchPattern {
    /** MatchPattern Constructor */
    pub fn new(search_config: &SearchConfig) 
                -> Result<MatchPattern, regex::Error> {
        let mut patterns: Vec<Regex> = Vec::new();

        for query in &search_config.queries {
            // apply word matching
            let query_string: String = if search_config.word_match {
                format!(r"\b{}\b", query)
            } else {
                format!(r"{}", query)
            };
            // apply case insensitive flag
            let query_string: String = if !search_config.case_sensitive {
                format!(r"(?i){}", query_string)
            } else {
                query_string
            };

            let regex = Regex::new(&query_string)?;
            patterns.push(regex);
        }

        Ok(
            MatchPattern { 
                patterns, 
                invert: search_config.invert_match
            }
        )
    }

    /** Checks if a given line matches with any pattern */
    pub fn matches(&self, line: &str) -> bool {
        // Check every pattern with each line
        for pattern in &self.patterns {
            if pattern.is_match(line) != self.invert {
                return true;
            }
        }

        return false;
   }
}