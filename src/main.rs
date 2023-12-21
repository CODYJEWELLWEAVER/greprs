extern crate greprs;
extern crate regex;

use std::env;
use std::process;
use std::io::prelude::*;

use greprs::config::Config;
use greprs::consts;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();
    
    let config: Config = Config::new(&args)
            .unwrap_or_else(|err: &str| { 
                writeln!(
                    &mut stderr,
                    "{} {}",
                    consts::ERR_MSG_PARSING_ARGS,
                    err,
                ).expect(consts::ERR_MSG_STD_ERR_WRITE);

                process::exit(1);
            });

    if let Err(err) = greprs::run(config) {
        writeln!(
            &mut stderr,
            "{} {}",
            consts::ERR_MSG_APP_ERR,
            err,
        ).expect(consts::ERR_MSG_STD_ERR_WRITE);

        process::exit(1);
    }
}