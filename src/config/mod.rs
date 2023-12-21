use std::collections::HashMap;

pub mod search_config;
mod parse;

use search_config::SearchConfig;

use crate::consts::ERR_MSG_USAGE_HINT;
use crate::consts::GREPRS_HELP_OPTION;
use crate::consts::GREPRS_VERSION_OPTION;

/**
 * Holds configuration for greprs::run to use.
 */
pub struct Config<'a> {
    pub search_config:  Option<SearchConfig<'a>>,
    pub info_config: Option<InfoConfig>
}

// Wraps search arguments.
pub struct SearchArgs<'a> {
    pub query: &'a str,
    pub content: &'a str,
}

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub enum OptionType {
    CaseInsensitive,
    InvertMatch,
    CountOutput,
    Unknown,
}

// Wraps option args and their associated values.
pub struct OptionArgs<'a> {
    pub options: HashMap<OptionType, Vec<&'a str>>
}

// Defines configurations for info::run()
pub enum InfoConfig {
    HELP,
    VERSION,
}

impl Config<'_> {
    // Constructor for command line args.
    // Parses cl args and queries enviroment variables.
    // Param: cl args: &[Strings]
    // Return: Result<config: Config, errMsg: &'static str>
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            if args.len() == 2 { // Check for a help option
                let help_option = String::from(GREPRS_HELP_OPTION);
                let version_option = String::from(GREPRS_VERSION_OPTION);
                // Check if a info option is present.
                let config = match &args[1] {
                    _ if &args[1] == &help_option => {
                        Ok(Config {
                            search_config: None, 
                            info_config: Some(InfoConfig::HELP)
                        })
                    },
                    _ if &args[1] == &version_option => {
                        Ok(Config {
                            search_config: None, 
                            info_config: Some(InfoConfig::VERSION)
                        })
                    },
                    _ => {
                        Err(
                            ERR_MSG_USAGE_HINT
                        )
                    }
                };

                return config;
            }
            // If no information option is passed in 
            // requires file and query arguments.
            return Err(
                ERR_MSG_USAGE_HINT
            );
        }
        
        // Parses arguments for search parameters: (query, content) 
        let search_args = parse::parse_search_args(&args)?;
        // Parses arguments for program options
        let option_args = parse::parse_option_args(&args)?;

        let search_config: SearchConfig = SearchConfig::new(search_args, option_args)?;

        Ok( 
            Config { 
                search_config: Some(search_config),
                info_config: None
            }
        )   
    }
}
