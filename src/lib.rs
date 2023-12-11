use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use std::env;

mod consts;

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

// Wraps command line arguments.
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // Constructor for command line args.
    // Parses cl args and queries enviroment variables.
    // Param: cl args: &[Strings]
    // Return: Result<config: Config, errMsg: &'static str>
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // Requires file and query arguments.
            return Err(
                "Not enough arguments."
            );
        }

        let query: &str = &args[1].clone();
        let filename: &str = &args[2].clone();

        // Check if search is case sensitive
        let case_sensitive: bool = parse_case_sensitive(args);

        Ok( 
            Config { 
                query: query.to_string(), 
                filename: filename.to_string(),
                case_sensitive: case_sensitive,
            }
        )   
    }
}

/**
 * Checks both enviroment variables and cl arguments to determine 
 * if search is case sensitive. CL arguments take priority over 
 * enviroment variables.
 */
fn parse_case_sensitive(args: &[String]) -> bool {
    // Check env var.
    let var_result = match env::var_os(consts::CASE_INSENSITIVE_VAR) {
        Some(s) => s == "0",
        None => false,
    };

    // Check for command line argument.
    let arg_result = if args.len() < 4 {
        // Default to eviroment var if 
        // no arg passed in.
        var_result
    } else {
        let case_insensitive: &str = &args[3].clone();
        if case_insensitive == "0" {
            // Overrides env var if false passed in as cli arg
            return true;
        } else if case_insensitive == "1" {
            // Override env var if true passed in as cli arg
            return false
        }
        else {
            // If non-sense arg is passed will ignore the value
            // and will default to case insensitive
            false
        }
    };

    var_result || arg_result
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