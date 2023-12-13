extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::*;

use greprs::config::Config;

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

    println!("Searching for '{}':", config.query);
    println!("In file {}:", config.filename);

    if let Err(e) = greprs::run(config) {
        writeln!(
            &mut stderr,
            "Application error: {}",
            e,
        ).expect("Could not write to stderr");

        process::exit(1);
    }
}