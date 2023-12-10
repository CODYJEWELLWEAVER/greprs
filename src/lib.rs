use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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
}

// Main search function.
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }

    results
}

// Wraps command line arguments.
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // Constructor for command line args.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // Requires file and query arguments.
            return Err(
                "Not enough arguments."
            );
        }

        let query: &str = &args[1].clone();
        let filename: &str = &args[2].clone();

        Ok( 
            Config { 
                query: query.to_string(), 
                filename: filename.to_string() 
            }
        )   
    }
}

// Run greprs
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search( &config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}