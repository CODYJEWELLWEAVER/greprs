use std::error::Error;

// Define modules to expose.
pub mod config;
pub mod consts;

use config::{Config, InfoConfig};
use config::search_config::SearchConfig;
use output::Output;

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

    let search_output: Output<'_> = search::run(&search_config)?;
    search_output.display();

    Ok(())
}