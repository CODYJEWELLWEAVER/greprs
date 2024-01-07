#[cfg(test)]
pub mod search_config_tests {
    use crate::config::OptionType;
    use crate::config::SearchArgs;
    use crate::config::search_config::SearchConfig;

    #[test]
    fn default_options() {
        let search_args = SearchArgs{queries: vec!("Hello"), files: vec!("res/test/poem.txt"), options: Vec::new()};
        let search_config = SearchConfig::new(search_args).unwrap();

        assert_eq!(search_config.queries, vec!("Hello"));
        assert_eq!(search_config.files, vec!("res/test/poem.txt"));
        assert_eq!(search_config.case_sensitive, true);
        assert_eq!(search_config.invert_match, false);
        assert_eq!(search_config.count_output, false);
    }

    #[test]
    fn case_insensitive() {
        let search_args = SearchArgs{queries: vec!("Hello"), files: vec!("res/test/poem.txt"), options: vec!(OptionType::CaseInsensitive)};
        let search_config = SearchConfig::new(search_args).unwrap();

        assert_eq!(search_config.queries, vec!("Hello"));
        assert_eq!(search_config.files, vec!("res/test/poem.txt"));
        assert_eq!(search_config.case_sensitive, false);
        assert_eq!(search_config.invert_match, false);
        assert_eq!(search_config.count_output, false);
    }

    #[test]
    fn invert_and_case_insensitive() {
        let mut options: Vec<OptionType> = Vec::new();
        options.push(OptionType::CaseInsensitive);
        options.push(OptionType::InvertMatch);
        let search_args = SearchArgs{queries: vec!("Hello"), files: vec!("res/test/poem.txt"), options};
        let search_config = SearchConfig::new(search_args).unwrap();

        assert_eq!(search_config.queries, vec!("Hello"));
        assert_eq!(search_config.files, vec!("res/test/poem.txt"));
        assert_eq!(search_config.case_sensitive, false);
        assert_eq!(search_config.invert_match, true);
        assert_eq!(search_config.count_output, false);
    }

    #[test]
    fn count_output_and_invert_match() {
        let mut options: Vec<OptionType> = Vec::new();
        options.push(OptionType::CountOutput);
        options.push(OptionType::InvertMatch);
        let search_args = SearchArgs{queries: vec!("Hello"), files: vec!("res/test/poem.txt"), options};
        let search_config = SearchConfig::new(search_args).unwrap();

        assert_eq!(search_config.queries, vec!("Hello"));
        assert_eq!(search_config.files, vec!("res/test/poem.txt"));
        assert_eq!(search_config.case_sensitive, true);
        assert_eq!(search_config.invert_match, true);
        assert_eq!(search_config.count_output, true);
    }
}