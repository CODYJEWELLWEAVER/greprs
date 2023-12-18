use crate::config::OptionArgs;
use crate::config::SearchArgs;

#[cfg(test)]
mod test {
    use super::parse_option_args;

    #[test]
    fn parse_options() {
        let test_args = ["-i".to_string(), "-d".to_string(), "--D".to_string(), "dummy value".to_string()];
        let option_args = parse_option_args(&test_args).unwrap();
        assert_eq!(option_args.options, test_args[..3]);
        assert_eq!(option_args.option_values, test_args[3..]);
    }
}

// Parses input args and maps option markups with
// their respective values. Option args are then passed into
// SearchConfig, OutputConfig, etc..
pub fn parse_option_args<'a>(args: &'a[String]) -> Result<OptionArgs, &'static str> {
    let mut options: Vec<&str> = Vec::new();
    let mut option_values: Vec<&str> = Vec::new();
    for i in 0..args.len() {
        // Parse options that take some sort of input for configuration.
        if args[i].starts_with("--") {
            if i == args.len() - 1 {
                return Err("Cannot parse options, run 'greprs help' for usage help.")
            }

            options.push(&args[i]);
            option_values.push(&args[i+1]);
        }
        // Parse options that need no additional input for configuration.
        else if args[i].starts_with("-") {
            options.push(&args[i]);
        }
    }

    Ok(OptionArgs{ options, option_values })
}

// Parses input args for query and content arguments
// Returns Err(msg) on failure and SearchArgs on success.
pub fn parse_search_args<'a>(args: &'a[String]) -> Result<SearchArgs, &'static str> {
    if args[2] != "in".to_string() {
        let query: &str = &args[1];
        let content: &str = &args[2]; 

        if query.is_empty() || content.is_empty() {
            return Err(
                "Error parsing query and content args. Run 'greprs help' for detailed information."
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