#[cfg(test)]
mod search_tests {
    use crate::config::search::SearchConfig;
    use crate::search::run;

    #[test]
    fn case_sensitive_search() {
        let search_config = SearchConfig {
            query: "a", 
            content: "Hello a\nA New World\n",
            case_sensitive: true,
            invert_match: false
        };
        assert_eq!(run(&search_config).unwrap(), ["Hello a"]);
    }

    #[test]
    fn case_insensitive_search() {
        let search_config = SearchConfig {
            query: "a", 
            content: "a Hello\nA New World\n",
            case_sensitive: false,
            invert_match: false
        };
        assert_eq!(run(&search_config).unwrap(), ["a Hello", "A New World"]);
    }

    #[test]
    fn invert_match_search() {
        let search_config = SearchConfig {
            query: "query", 
            content: "a Hello\nA New World\n",
            case_sensitive: false,
            invert_match: true
        };
        assert_eq!(run(&search_config).unwrap(), ["a Hello", "A New World"]);

        let search_config = SearchConfig {
            query: "a", 
            content: "a Hello\nA New World\n",
            case_sensitive: true,
            invert_match: true
        };
        assert_eq!(run(&search_config).unwrap(), ["A New World"]);

        let search_config = SearchConfig {
            query: "test", 
            content: "A test!\nNot one!\nAnother Test!",
            case_sensitive: false,
            invert_match: true
        };
        assert_eq!(run(&search_config).unwrap(), ["Not one!"]);
    }
}