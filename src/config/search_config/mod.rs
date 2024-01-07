use crate::config::OptionType;
use super::SearchArgs;

mod test;

/**
Contains neccessary information to run search
* queries: patterns and strings to search files with\
* files: names of files to search
* case_sensitive: false if search should ignore case
* invert_match: true if should invert matching
* count_output: true if normal output should be replaced with 
count of matching lines in each file.
* word_match: true if word matching logic should be applied.
 */
#[derive(Debug)]
#[derive(PartialEq)]
pub struct SearchConfig<'a> {
    pub queries: Vec<&'a str>,
    pub files: Vec<&'a str>,
    pub case_sensitive: bool,
    pub invert_match: bool,
    pub count_output: bool,
    pub word_match: bool,
}

impl SearchConfig<'_> {
    /** SearchConfig Constructor */
    pub fn new<'a>(
        search_args: SearchArgs<'a>
    ) -> Result<SearchConfig<'a>, &'static str> {
        let queries = search_args.queries;
        let files = search_args.files;
        let options = search_args.options;

        let case_sensitive: bool 
        = if options.contains(&OptionType::CaseInsensitive) {
                false
        } else { true };

        let invert_match: bool 
        = if options.contains(&OptionType::InvertMatch) {
            true
        } else { false };

        let count_output: bool 
        = if options.contains(&OptionType::CountOutput) {
            true
        } else { false };

        let word_match: bool
        = if options.contains(&OptionType::WordMatch) {
            true
        } else { false };
        
        Ok(
            SearchConfig {
                queries,
                files,
                case_sensitive,
                invert_match,
                count_output,
                word_match
            }
        )
    }
}