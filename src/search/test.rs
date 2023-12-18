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
        };
        assert_eq!(run(&search_config), ["Hello a"]);
    }

    #[test]
    fn case_insensitive_search() {
        let search_config = SearchConfig {
            query: "a", 
            content: "a Hello\nA New World\n",
            case_sensitive: false,
        };
        assert_eq!(run(&search_config), ["a Hello", "A New World"]);
    }
}