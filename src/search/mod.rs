// Case sensitive search function.
// Params: query: Config.query - user config,
//         contents: Config.contents - file to search.
// List of matching lines.
pub fn case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
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
pub fn case_insensitive<'a>(
    query: &str, 
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}