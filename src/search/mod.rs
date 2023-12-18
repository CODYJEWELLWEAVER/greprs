use crate::config::search::SearchConfig;

mod test;

// Case sensitive search function.
// Params: query: Config.query - user config,
//         contents: Config.contents - file to search.
// List of matching lines.
fn case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// Case insensitvie search function
// Params: query: Config.query - user config,
//         contents: Config.contents - file to search.
// List of matching lines.
fn case_insensitive<'a>(
    query: &str, 
    content: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// Runs the search given the config parameter. 
// Returns output list.
pub fn run<'a>(config: &SearchConfig<'a>) -> Vec<&'a str>{
    return if config.case_sensitive {
        case_sensitive(config.query, config.content)
    }
    else {
        case_insensitive(config.query, config.content)
    }
}