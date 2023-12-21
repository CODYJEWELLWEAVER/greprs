#[cfg(test)]
mod matcher_tests{
    use crate::config::search::SearchConfig;
    use crate::matcher::MatchPattern;
    use crate::matcher::matches;

    #[test]
    fn basic_match_test() {
        let search_config = SearchConfig{
            query: "hello",
            content: "this is a test! hello!",
            case_sensitive: true,
            invert_match: false,
            count_output: false,
        };
        let match_pattern = MatchPattern::new(&search_config).expect("basic_match_test panicked!");
        assert!(matches(&search_config.content, &match_pattern));
        assert_ne!(matches("Hello", &match_pattern), true);
    }

    #[test]
    fn case_insensitive_match_test() {
        let search_config = SearchConfig{
            query: "Hello",
            content: "this is a test! hello!",
            case_sensitive: false,
            invert_match: false,
            count_output: false,
        };
        let match_pattern = MatchPattern::new(&search_config).expect("case_insensitive_match_test panicked!");
        assert!(matches(&search_config.content, &match_pattern));
        assert_eq!(matches("This is a test Hello!", &match_pattern), true);
    }
}
