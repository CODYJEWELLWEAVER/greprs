pub mod search;

use search::SearchConfig;

pub struct Config<'a> {
    pub search_config:  SearchConfig<'a>,
}

pub struct SearchArgs<'a> {
    pub query: &'a str,
    pub content: &'a str,
}

pub struct OptionArgs<'a> {
    pub options: Vec<&'a String>,
    pub option_values: Vec<&'a String>
}

impl Config<'_> {
    // Constructor for command line args.
    // Parses cl args and queries enviroment variables.
    // Param: cl args: &[Strings]
    // Return: Result<config: Config, errMsg: &'static str>
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // Requires file and query arguments.
            return Err(
                "Not enough arguments."
            );
        }

        let search_args = Self::parse_search_args(&args)?;
        let option_args = Self::parse_option_args(&args)?;

        // Check if search is case sensitive
        let search_config: SearchConfig = SearchConfig::new(search_args, option_args);

        Ok( 
            Config { 
                search_config,
            }
        )   
    }

    // Parses input args and maps option markups with
    // their respective values. Option args are then passed into
    // SearchConfig, OutputConfig, etc..
    fn parse_option_args<'a>(args: &'a[String]) -> Result<OptionArgs, &'static str> {
        let mut options = Vec::new();
        let mut option_values = Vec::new();
        for i in 0..args.len() {
            // Parse options that take some sort of input for configuration.
            if args[i].starts_with("--") && i != args.len() - 1 {
                options.push(&args[i]);
                option_values.push(&args[i+1]);
            }
            else {
                let err_msg = String::from(&args[i]);
                return Err("No value passed in for option: {err_msg}")
            }
            // Parse options that need no additional input for configuration.
            if args[i].starts_with("-") {
                options.push(&args[i]);
            }
        }

        Ok(OptionArgs{ options, option_values })
    }

    // Parses input args for query and content arguments.
    fn parse_search_args<'a>(args: &'a[String]) -> Result<SearchArgs, &'static str> {
        if args[2] != "in".to_string() {
            let query: &str = &args[1];
            let content: &str = &args[3]; 

            if query.is_empty() || content.is_empty() {
                return Err(
                    "Error parsing query and content args. Run greprs help for detailed information."
                )
            }
            else {
                return Ok(SearchArgs{query, content})
            }
        }
        else {
            // Define query arguments using <query> in <content> syntax.
            // Default for content is a filename.
            let query: &str = &args[1];
            let content: &str = &args[3];
            Ok(SearchArgs{query, content})
        }
    }
}
