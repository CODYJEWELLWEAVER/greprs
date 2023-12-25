use regex::Regex;
use crate::SearchConfig;

mod test;

pub struct MatchPattern {
    pub patterns: Vec<Regex>,
    pub invert: bool
}

impl MatchPattern {
    pub fn new(search_config: &SearchConfig) 
                -> Result<MatchPattern, regex::Error> {
        let mut patterns: Vec<Regex> = Vec::new();

        for query in &search_config.queries {
            let query_string: String = if search_config.case_sensitive {
                format!(r"{}", query)
            } else {
                format!(r"(?i){}", query)
            };
            let regex = Regex::new(&query_string)?;
            patterns.push(regex);
        }

        Ok(
            MatchPattern { 
                patterns: patterns, 
                invert: search_config.invert_match
            }
        )
    }

    // Checks if a given line matches with any pattern 
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