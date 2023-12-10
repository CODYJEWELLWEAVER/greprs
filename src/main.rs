extern crate greprs;

use std::env;
use std::process;

use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config: Config = Config::new(&args)
            .unwrap_or_else(|err: &str| { 
                // Exit process on config init error.
                println!("Problem parsing arguments: {}", err);
                process::exit(1);
            });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}