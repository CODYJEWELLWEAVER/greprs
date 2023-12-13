use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use config::Config;


mod consts;

pub mod config;

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fase, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents),
        )
    }
}

// Case sensitive search function.
// Params: query: Config.query - user config,
//         contents: Config.contents - file to search.
// List of matching lines.
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
fn search_case_insensitive<'a>(
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

// Run greprs
// Param: config : Config - user specified configuration.
// Return: () On success - Error implementing class on failure.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Opens file.
    let mut f = File::open(config.filename)?;

    // Reads file.
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // Get search results.
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // Display search results.
    for line in results {
        println!("{}", line);
    }

    Ok(())
}