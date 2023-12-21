#[cfg(test)]
mod search_tests {
    use crate::config::search::SearchConfig;
    use crate::output::{OutputType, Output};
    use crate::search::run;

    #[test]
    fn case_sensitive_search() {
        let search_config = SearchConfig {
            query: "a", 
            content: "Hello a\nA New World\n",
            case_sensitive: true,
            invert_match: false,
            count_output: false,
        };
        let output = Output::new(
            Some(&search_config),
            vec!("Hello a"),
            OutputType::SEARCH
        );
        assert_eq!(run(&search_config).unwrap(), output);
    }

    #[test]
    fn case_insensitive_search() {
        let search_config = SearchConfig {
            query: "a", 
            content: "a Hello\nA New World\n",
            case_sensitive: false,
            invert_match: false,
            count_output: false,
        };
        let output = Output::new(
            Some(&search_config),
            vec!("a Hello", "A New World"),
            OutputType::SEARCH
        );
        assert_eq!(run(&search_config).unwrap(), output);
    }

    #[test]
    fn invert_match_search() {
        let search_config = SearchConfig {
            query: "query", 
            content: "a Hello\nA New World\n",
            case_sensitive: false,
            invert_match: true,
            count_output: false,
        };
        let output = Output::new(
            Some(&search_config),
            vec!("a Hello", "A New World"),
            OutputType::SEARCH
        );
        assert_eq!(run(&search_config).unwrap(), output);

        let search_config = SearchConfig {
            query: "a", 
            content: "a Hello\nA New World\n",
            case_sensitive: true,
            invert_match: true,
            count_output: false,
        };
        let output = Output::new(
            Some(&search_config),
            vec!("A New World"),
            OutputType::SEARCH
        );
        assert_eq!(run(&search_config).unwrap(), output);

        let search_config = SearchConfig {
            query: "test", 
            content: "A test!\nNot one!\nAnother Test!",
            case_sensitive: false,
            invert_match: true,
            count_output: false,
        };
        let output = Output::new(
            Some(&search_config),
            vec!("Not one!"),
            OutputType::SEARCH
        );
        assert_eq!(run(&search_config).unwrap(), output);
    }
}