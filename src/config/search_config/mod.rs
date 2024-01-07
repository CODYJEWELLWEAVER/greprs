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
#[derive(Clone)]
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
        
        Ok(
            SearchConfig {
                queries,
                files,
                case_sensitive: !options.contains(&OptionType::CaseInsensitive),
                invert_match: options.contains(&OptionType::InvertMatch),
                count_output: options.contains(&OptionType::CountOutput),
                word_match: options.contains(&OptionType::WordMatch)
            }
        )
    }
}