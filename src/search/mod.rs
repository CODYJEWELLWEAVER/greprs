use crate::config::search::SearchConfig;
use crate::matcher;
use crate::output::{Output, OutputType};

mod test;

// Runs the search given the config parameter
// and returns the output list.
pub fn run<'a>(
    search_config: &'a SearchConfig<'a>
) -> Result<Output<'a>, regex::Error> {
    // TODO: MOVE FILE OPENING HERE
    let content: &str = search_config.content;

    let mut search_results = Vec::new();
    let match_pattern = matcher::MatchPattern::new(search_config)?;

    for line in content.lines() {
        if matcher::matches(line, &match_pattern) {
            search_results.push(line);
        }
    }

    let output_type = if search_config.count_output {
        OutputType::SEARCH_COUNT
    }
    else {
        OutputType::SEARCH
    };

    let search_output = Output::new(
        Some(search_config),
        search_results,
        output_type
    );

    Ok(search_output)
}