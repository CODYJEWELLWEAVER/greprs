use std::collections::HashMap;
use crate::config::OptionType;
use super::SearchArgs;
use super::OptionArgs;


mod test;

/**
Contains neccessary information to run search
* queries: patterns and strings to search files with\
* files: names of files to search
* case_sensitive: false if search should ignore case
* invert_match: true if should invert matching
* count_output: true if normal output should be replaced with 
count of matching lines in each file.
 */
#[derive(Debug)]
#[derive(PartialEq)]
pub struct SearchConfig<'a> {
    pub queries: Vec<&'a str>,
    pub files: Vec<&'a str>,
    pub case_sensitive: bool,
    pub invert_match: bool,
    pub count_output: bool,
}

impl SearchConfig<'_> {
    /** SearchConfig Constructor */
    pub fn new<'a>(
        search_args: SearchArgs<'a>, 
        option_args: OptionArgs
    ) -> Result<SearchConfig<'a>, &'static str> {
        let queries = search_args.queries;
        let files = search_args.files;
        let options: HashMap<OptionType, Vec<& str>> = option_args.options;

        let case_sensitive: bool 
        = if options.contains_key(&OptionType::CaseInsensitive) {
                // A case sensitive option overrides
                // a case insensitive option.
                false
        } else { true };

        let invert_match: bool 
        = if options.contains_key(&OptionType::InvertMatch) {
            true
        } else { false };

        let count_output: bool 
        = if options.contains_key(&OptionType::CountOutput) {
            true
        } else { false };
        
        Ok(
            SearchConfig {
                queries,
                files,
                case_sensitive,
                invert_match,
                count_output
            }
        )
    }
}