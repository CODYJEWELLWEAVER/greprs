use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// Define modules to expose.
pub mod config;
pub mod consts;

use config::{Config, InfoConfig};
use config::search::SearchConfig;
use output::Output;

// Internal modules
mod matcher;
mod search;
mod info;
mod output;

// Run greprs
// Param: config : Config - user specified configuration.
// Return: () On success - Error implementing class on failure.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let info_config: Option<InfoConfig> = config.info_config;

    // If a configuration for an "info"
    // run exists run it and exit.
    match info_config {
        Some(cfg) => {
            let info_ouput: Output<'_> = info::run(cfg);
            info_ouput.display();
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
        count_output: search_config.count_output,
    };

    let search_output: Output<'_> = search::run(&search_config)?;
    search_output.display();

    Ok(())
}