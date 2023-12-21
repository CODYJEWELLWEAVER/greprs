use super::SearchArgs;
use super::OptionArgs;
use super::parse;

mod test;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct SearchConfig<'a> {
    pub query: &'a str,
    pub content: &'a str,
    pub case_sensitive: bool,
    pub invert_match: bool,
    pub count_output: bool,
}

impl SearchConfig<'_> {
    pub fn new<'a>(
        search_args: SearchArgs<'a>, 
        option_args: OptionArgs
    ) -> Result<SearchConfig<'a>, &'static str> {
        let query = search_args.query;
        let content = search_args.content;

        // Parse Search Options
        let case_sensitive: bool = parse::parse_case_sensitive(&option_args);
        let invert_match: bool = parse::parse_invert_match(&option_args);

        // Search Output Options
        let count_output: bool = parse::parse_count_option(&option_args);
        
        Ok(
            SearchConfig {
                query,
                content,
                case_sensitive,
                invert_match,
                count_output
            }
        )
    }
}