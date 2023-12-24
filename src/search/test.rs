#[cfg(test)]
mod search_tests {
    use crate::config::search_config::SearchConfig;
    use crate::output::{OutputType, Output};
    use crate::search::run;

    #[test]
    fn case_sensitive_search() {
        let search_config = SearchConfig {
            query: "a", 
            files: vec!("res/test/haiku.txt"),
            case_sensitive: true,
            invert_match: false,
            count_output: false,
        };
        let output = Output::new(
            Some(&search_config),
            vec!(Box::new("is a world of dew,".to_owned()), Box::new("and yet, and yet.".to_owned())),
            OutputType::Search
        );
        assert_eq!(run(&search_config).unwrap(), output);
    }

    #[test]
    fn case_insensitive_search() {
        let search_config = SearchConfig {
            query: "is", 
            files: vec!("res/test/haiku.txt"),
            case_sensitive: false,
            invert_match: false,
            count_output: false,
        };
        let output = Output::new(
            Some(&search_config),
            vec!(Box::new("This world of dew".to_string()), Box::new("is a world of dew,".to_string())),
            OutputType::Search
        );
        assert_eq!(run(&search_config).unwrap(), output);
    }

    #[test]
    fn invert_match_search() {
        let search_config = SearchConfig {
            query: "dew", 
            files: vec!("res/test/haiku.txt"),
            case_sensitive: false,
            invert_match: true,
            count_output: false,
        };
        let output = Output::new(
            Some(&search_config),
            vec!(Box::new("and yet, and yet.".to_string())),
            OutputType::Search
        );
        assert_eq!(run(&search_config).unwrap(), output);

        let search_config = SearchConfig {
            query: "a", 
            files: vec!("res/test/haiku.txt"),
            case_sensitive: true,
            invert_match: true,
            count_output: false,
        };
        let output = Output::new(
            Some(&search_config),
            vec!(Box::new("This world of dew".to_string())),
            OutputType::Search
        );
        assert_eq!(run(&search_config).unwrap(), output);

        let search_config = SearchConfig {
            query: "test", 
            files: vec!("res/test/haiku.txt"),
            case_sensitive: false,
            invert_match: true,
            count_output: false,
        };
        let output = Output::new(
            Some(&search_config),
            vec!(Box::new("This world of dew".to_string()),
                                Box::new("is a world of dew,".to_string()),
                                Box::new("and yet, and yet.".to_string())),
            OutputType::Search
        );
        assert_eq!(run(&search_config).unwrap(), output);
    }
}