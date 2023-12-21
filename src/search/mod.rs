use crate::config::search::SearchConfig;
use crate::matcher;

mod test;

// Runs the search given the config parameter
// and returns the output list.
pub fn run<'a>(
    search_config: &SearchConfig<'a>
) -> Result<Vec<&'a str>, regex::Error> {
    // TODO: MOVE FILE OPENING HERE
    let content: &str = search_config.content;

    let mut results = Vec::new();
    let match_pattern = matcher::MatchPattern::new(search_config)?;

    for line in content.lines() {
        if matcher::matches(line, &match_pattern) {
            results.push(line);
        }
    }

    Ok(results)
}