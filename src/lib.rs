use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// Import greprs modules.
use config::Config;

// Define modules to expose.
pub mod config;
mod consts;
mod search;
////////////////////////
// Unit Tests for greprs
////////////////////////
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
            search::case_sensitive(query, contents)
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
            search::case_sensitive(query, contents)
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
            search::case_insensitive(query, contents),
        )
    }
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
        search::case_sensitive(&config.query, &contents)
    } else {
        search::case_insensitive(&config.query, &contents)
    };

    // Display search results.
    for line in results {
        println!("{}", line);
    }

    Ok(())
}