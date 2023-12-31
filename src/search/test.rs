#[cfg(test)]
mod search_tests {
    use crate::config::search_config::SearchConfig;
    use crate::output::{OutputType, Output};
    use crate::search::run;
    use std::collections::HashMap;

    #[test]
    fn case_sensitive_search() {
        let search_config = SearchConfig {
            queries: vec!("a"), 
            files: vec!("res/test/haiku.txt"),
            case_sensitive: true,
            invert_match: false,
            count_output: false,
            word_match: false,
        };
        let mut output_content = HashMap::new();
        output_content.insert("res/test/haiku.txt".to_string(), vec!("is a world of dew,".to_owned(), "and yet, and yet.".to_owned()));
        let output = Output::new(
            Some(search_config.clone()),
            output_content,
            OutputType::Search
        );
        assert_eq!(run(search_config).unwrap(), output);
    }

    #[test]
    fn case_insensitive_search() {
        let search_config = SearchConfig {
            queries: vec!("is"), 
            files: vec!("res/test/haiku.txt"),
            case_sensitive: false,
            invert_match: false,
            count_output: false,
            word_match: false,
        };
        let mut output_content = HashMap::new(); 
        output_content.insert("res/test/haiku.txt".to_string(), vec!("This world of dew".to_owned(),"is a world of dew,".to_owned()));
        let output = Output::new(
            Some(search_config.clone()),
            output_content,
            OutputType::Search
        );
        assert_eq!(run(search_config).unwrap(), output);
    }

    #[test]
    fn invert_match_search() {
        let search_config = SearchConfig {
            queries: vec!("dew"), 
            files: vec!("res/test/haiku.txt"),
            case_sensitive: false,
            invert_match: true,
            count_output: false,
            word_match: false,
        };
        let mut output_content = HashMap::new();
        output_content.insert("res/test/haiku.txt".to_string(), vec!("and yet, and yet.".to_owned()));
        let output = Output::new(
            Some(search_config.clone()),
            output_content,
            OutputType::Search
        );
        assert_eq!(run(search_config).unwrap(), output);

        let search_config = SearchConfig {
            queries: vec!("a"), 
            files: vec!("res/test/haiku.txt"),
            case_sensitive: true,
            invert_match: true,
            count_output: false,
            word_match: false,
        };
        let mut output_content = HashMap::new();
        output_content.insert("res/test/haiku.txt".to_string(), vec!("This world of dew".to_string()));
        let output = Output::new(
            Some(search_config.clone()),
            output_content,
            OutputType::Search
        );
        assert_eq!(run(search_config).unwrap(), output);

        let search_config = SearchConfig {
            queries: vec!("test"), 
            files: vec!("res/test/haiku.txt"),
            case_sensitive: false,
            invert_match: true,
            count_output: false,
            word_match: false,
        };
        let mut output_content = HashMap::new();
        output_content.insert(
            "res/test/haiku.txt".to_string(), 
            vec!("This world of dew".to_string(),
                "is a world of dew,".to_string(),
                "and yet, and yet.".to_string()
            )
        );
        let output = Output::new(
            Some(search_config.clone()),
            output_content,
            OutputType::Search
        );
        assert_eq!(run(search_config).unwrap(), output);
    }
}