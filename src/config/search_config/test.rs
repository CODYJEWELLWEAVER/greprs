#[cfg(test)]
pub mod search_config_tests {
    use crate::config::OptionArgs;
    use crate::config::OptionType;
    use crate::config::SearchArgs;
    use crate::config::search_config::SearchConfig;
    use std::collections::HashMap;

    #[test]
    fn default_options() {
        let search_args = SearchArgs{query: "Hello", content: "Hello World"};
        let options_map: HashMap<OptionType, Vec<&str>> = HashMap::new();
        let option_args = OptionArgs {
            options: options_map
        };
        let search_config = SearchConfig::new(search_args, option_args).unwrap();

        assert_eq!(search_config.query, "Hello");
        assert_eq!(search_config.content, "Hello World");
        assert_eq!(search_config.case_sensitive, true);
        assert_eq!(search_config.invert_match, false);
        assert_eq!(search_config.count_output, false);
    }

    #[test]
    fn case_insensitive() {
        let search_args = SearchArgs{query: "Hello", content: "Hello World"};
        let mut options_map: HashMap<OptionType, Vec<&str>> = HashMap::new();
        options_map.insert(OptionType::CaseInsensitive, Vec::new());
        let option_args = OptionArgs {
            options: options_map
        };
        let search_config = SearchConfig::new(search_args, option_args).unwrap();

        assert_eq!(search_config.query, "Hello");
        assert_eq!(search_config.content, "Hello World");
        assert_eq!(search_config.case_sensitive, false);
        assert_eq!(search_config.invert_match, false);
        assert_eq!(search_config.count_output, false);
    }

    #[test]
    fn invert_and_case_insensitive() {
        let search_args = SearchArgs{query: "Hello", content: "Hello World"};
        let mut options_map: HashMap<OptionType, Vec<&str>> = HashMap::new();
        options_map.insert(OptionType::CaseInsensitive, Vec::new());
        options_map.insert(OptionType::InvertMatch, Vec::new());
        let option_args = OptionArgs {
            options: options_map
        };
        let search_config = SearchConfig::new(search_args, option_args).unwrap();

        assert_eq!(search_config.query, "Hello");
        assert_eq!(search_config.content, "Hello World");
        assert_eq!(search_config.case_sensitive, false);
        assert_eq!(search_config.invert_match, true);
        assert_eq!(search_config.count_output, false);
    }

    #[test]
    fn count_output_and_invert_match() {
        let search_args = SearchArgs{query: "Hello", content: "Hello World"};
        let mut options_map: HashMap<OptionType, Vec<&str>> = HashMap::new();
        options_map.insert(OptionType::CountOutput, Vec::new());
        options_map.insert(OptionType::InvertMatch, Vec::new());
        let option_args = OptionArgs {
            options: options_map
        };
        let search_config = SearchConfig::new(search_args, option_args).unwrap();

        assert_eq!(search_config.query, "Hello");
        assert_eq!(search_config.content, "Hello World");
        assert_eq!(search_config.case_sensitive, true);
        assert_eq!(search_config.invert_match, true);
        assert_eq!(search_config.count_output, true);
    }
}