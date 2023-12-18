use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// Define modules to expose.
pub mod config;
use config::Config;
use config::search::SearchConfig;

// Internal modules.
mod consts;
mod search;

//////////////
// TEST Module
//////////////
#[cfg(test)]
mod test {
    use super::*;
}

// Run greprs
// Param: config : Config - user specified configuration.
// Return: () On success - Error implementing class on failure.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let search_config = config.search_config;
    // Opens file.
    let mut f = File::open(search_config.content)?;

    // Reads file.
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let search_results = search::run(search_config);

    // Display search results.
    for line in search_results {
        println!("{}", line);
    }

    Ok(())
}

/* Print Search Configuration Details */
pub fn print_search_config(config: &SearchConfig) {
    println!("Searching for: '{}'", config.query);
    println!("In: {}", config.content);
}