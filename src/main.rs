use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

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

    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // Constructor
    fn new(args: &[String]) -> Result<Config, &'static str> {
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