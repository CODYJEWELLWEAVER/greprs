use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// Define modules to expose.
pub mod config;
mod matcher;
use config::{Config, InfoConfig};
use config::search::SearchConfig;

// Internal modules.
mod consts;
mod search;
mod info;

//////////////
// TEST Module
//////////////
#[cfg(test)]
mod test {
}

// Run greprs
// Param: config : Config - user specified configuration.
// Return: () On success - Error implementing class on failure.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let info_config: Option<InfoConfig> = config.info_config;

    // If a configuration for an "info"
    // run exists run it and exit.
    match info_config {
        Some(cfg) => {
            let info_results = info::run(cfg);
            print_output(info_results);
            return Ok(())
        },
        None => {}
    }
    // Otherwise default to running a search configuration
    let search_config: SearchConfig = match config.search_config {
        Some(cfg) => cfg,
        // Return Error if no config found.
        None => return Err(Box::from("No search configuration! Exiting..."))
    };
    
    // Opens file.
    let mut file = File::open(search_config.content)?;

    // Reads file.
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    // Update search config with content from source
    let search_config = SearchConfig {
        query: search_config.query,
        content: &file_contents,
        case_sensitive: search_config.case_sensitive,
        invert_match: search_config.invert_match,
    };

    let search_results = search::run(&search_config)?;

    print_search_config(&search_config);

    // Display search results.
    print_output(search_results);

    Ok(())
}

pub fn print_output(run_results: Vec<& str>) {
    for line in run_results {
        println!("{}", line);
    }
}

/* Print Search Configuration Details */
pub fn print_search_config(config: &SearchConfig) {
    println!("Searching for: '{}'", config.query);
    println!("In: {}", config.content);
}