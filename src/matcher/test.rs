#[cfg(test)]
mod matcher_tests{
    use crate::config::search_config::SearchConfig;
    use crate::matcher::MatchPattern;

    #[test]
    fn basic_match_test() {
        let search_config = SearchConfig{
            queries: vec!("This"),
            files: vec!("res/test/haiku.txt"),
            case_sensitive: true,
            invert_match: false,
            count_output: false,
            word_match: false,
        };
        let match_pattern = MatchPattern::new(&search_config).expect("basic_match_test panicked!");
        assert!(match_pattern.matches("This world of dew"));
        assert_ne!(match_pattern.matches("this"), true);
    }

    #[test]
    fn case_insensitive_match_test() {
        let search_config = SearchConfig{
            queries: vec!("Hello"),
            files: vec!("res/test/poem.txt"),
            case_sensitive: false,
            invert_match: false,
            count_output: false,
            word_match: false,
        };
        let match_pattern = MatchPattern::new(&search_config).expect("case_insensitive_match_test panicked!");
        assert!(match_pattern.matches("I am not here, hello!"));
        assert_eq!(match_pattern.matches("This is a test Hello!"), true);
    }
}
