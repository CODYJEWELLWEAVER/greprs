extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::*;

use greprs::config::Config;
use greprs::print_search_config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();
    
    let config: Config = Config::new(&args)
            .unwrap_or_else(|err: &str| { 
                // Exit process on config init error.
                writeln!(
                    &mut stderr,
                    "Problem parsing arguments: {}",
                    err,
                ).expect("Could not write to stderr");
                process::exit(1);
            });

    print_search_config(&config.search_config);

    if let Err(e) = greprs::run(config) {
        writeln!(
            &mut stderr,
            "Application error: {}",
            e,
        ).expect("Could not write to stderr");

        process::exit(1);
    }
}