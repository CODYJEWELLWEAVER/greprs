#[cfg(test)]
pub mod info_tests {
    use std::collections::HashMap;
    use crate::consts::{self, };
    use crate::info::{self, InfoConfig};
    use crate::Config;
    use crate::output::{OutputType, Output};
    use crate::info::{HELP_KEY, VERSION_KEY};

    #[test]
    fn info_run_version() {
        let mut output_content = HashMap::new();
        let version_output = match consts::VERSION {
            Some(version_string) => {
                output_content.insert(VERSION_KEY.to_string(), vec!(Box::new(version_string.to_string())));
                Output::new(None, output_content, OutputType::Info)
            },
            None => {
                output_content.insert(HELP_KEY.to_string(), vec!(Box::new(consts::UNKNOWN_VERSION.to_string())));
                Output::new(None, output_content, OutputType::Info)
            }
        };
        assert_eq!(info::run(InfoConfig::Version), version_output);
    }

    #[test]
    fn greprs_run_version() {
        let greprs_config = Config{info_config: Some(InfoConfig::Version), search_config:None};
        let run_result = super::super::super::run(greprs_config).unwrap();
        assert_eq!((), run_result);
    }
}