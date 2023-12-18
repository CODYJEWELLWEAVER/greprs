use crate::config::search::SearchConfig;

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

pub fn run<'a>(config: SearchConfig<'a>) -> Vec<&'a str>{
    if config.case_sensitive {
        return case_sensitive(config.query, config.content)
    }
    else {
        return case_insensitive(config.query, config.content)
    }
}