use regex::Regex;
use crate::SearchConfig;

mod test;

pub struct MatchPattern {
    pub pattern: Regex,
    pub invert: bool
}

impl MatchPattern {
    pub fn new(search_config: &SearchConfig) 
                -> Result<MatchPattern, regex::Error> {
        // Use query to create a regex pattern that matches
        let mut query_string: String = format!(r"({})", search_config.query);
        // Add case insensitivity flag.
        if !search_config.case_sensitive {
            query_string = format!(r"(?i){}", &query_string);
        }
        let regex = Regex::new(&query_string)?;
        Ok(
            MatchPattern { 
                pattern: regex, 
                invert: search_config.invert_match
            }
        )
    }
}

pub fn matches(line: &str,
     match_pattern: &MatchPattern) -> bool {
    return match_pattern.pattern
            .is_match(line) != match_pattern.invert
}