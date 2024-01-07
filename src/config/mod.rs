pub mod search_config;
mod parse;

use search_config::SearchConfig;

use crate::consts::{ERR_MSG_USAGE_HINT, GREPRS_HELP_OPTION_0,
GREPRS_HELP_OPTION_1, GREPRS_VERSION_OPTION_0, GREPRS_VERSION_OPTION_1 };


/**
 * Holds configuration for greprs::run to use.
 */
pub struct Config<'a> {
    pub search_config:  Option<SearchConfig<'a>>,
    pub info_config: Option<InfoConfig>
}

/**
Wraps search arguments.
*/
pub struct SearchArgs<'a> {
    pub files: Vec<&'a str>,
    pub queries: Vec<&'a str>,
    pub options: Vec<OptionType>
}

/**
Encodes cli option types
 */
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub enum OptionType {
    CaseInsensitive,
    InvertMatch,
    CountOutput,
    WordMatch,
    Unknown,
}

/**
Defines configurations for info::run()
*/
pub enum InfoConfig {
    Help,
    Version,
}

impl SearchArgs<'_> {
    pub fn new<'a>(
        queries: Vec<&'a str>,
        files: Vec<&'a str>,
        options: Vec<OptionType>
    ) -> SearchArgs<'a> {
        SearchArgs {
            queries,
            files,
            options
        }
    }
}

impl Config<'_> {
    /**
    Constructor for command line args.
    Parses cl args and queries enviroment variables.
    * param: cl args: &\[Strings\]
    * return: Result<config: Config, errMsg: &'static str>
    */
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            if args.len() == 2 { // Check for a help option
                let help_option_0 = String::from(GREPRS_HELP_OPTION_0);
                let help_option_1 = String::from(GREPRS_HELP_OPTION_1);
                let version_option_0 = String::from(GREPRS_VERSION_OPTION_0);
                let version_option_1 = String::from(GREPRS_VERSION_OPTION_1);
                // Check if a info option is present.
                let config = match &args[1] {
                    _ if &args[1] == &help_option_0 || 
                         &args[1] == &help_option_1 => {
                        Ok(Config {
                            search_config: None, 
                            info_config: Some(InfoConfig::Help)
                        })
                    },
                    _ if &args[1] == &version_option_0 ||
                         &args[1] == &version_option_1 => {
                        Ok(Config {
                            search_config: None, 
                            info_config: Some(InfoConfig::Version)
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
        // let search_args = parse::parse_search_args(&args)?;
        // Parses arguments for program options
        // let option_args = parse::parse_option_args(&args)?;
        let search_args: SearchArgs = parse::parse_arguments(args)?;

        let search_config: SearchConfig = SearchConfig::new(search_args)?;

        Ok( 
            Config { 
                search_config: Some(search_config),
                info_config: None
            }
        )   
    }
}
