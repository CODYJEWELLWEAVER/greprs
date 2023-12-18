#[cfg(test)]
pub mod search_config_tests {
    use crate::config::OptionArgs;
    use crate::config::SearchArgs;
    use crate::config::search;
    use crate::config::search::SearchConfig;

    #[test]
    fn parse_sensitive() {
        let option_args = OptionArgs {options: vec![&"-ignore-case"], option_values: [].to_vec()};
        assert_eq!(search::parse_case_sensitive(option_args), false);

        let option_args = OptionArgs {options: vec![&"-no-ignore-case"], option_values: [].to_vec()};
        assert_eq!(search::parse_case_sensitive(option_args), true);

        let option_args = OptionArgs {options: vec![&"-i"], option_values: [].to_vec()};
        assert_eq!(search::parse_case_sensitive(option_args), false);
    }

    #[test]
    fn search_config_create() {
        let search_args = SearchArgs{query: "Hello", content: "Hello World"};
        let option_args = OptionArgs {options: vec![&"-no-ignore-case"], option_values: [].to_vec()};
        let search_config = SearchConfig::new(search_args, option_args).unwrap();

        assert_eq!(search_config.query, "Hello");
        assert_eq!(search_config.content, "Hello World");
        assert_eq!(search_config.case_sensitive, true);

        let search_args = SearchArgs{query: "Orange", content: "Doorhinge"};
        let option_args = OptionArgs {options: vec![&"-ignore-case"], option_values: [].to_vec()};
        let search_config = SearchConfig::new(search_args, option_args).unwrap();

        assert_eq!(search_config.query, "Orange");
        assert_eq!(search_config.content, "Doorhinge");
        assert_eq!(search_config.case_sensitive, false);
    }
}