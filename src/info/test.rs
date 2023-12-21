#[cfg(test)]
pub mod info_tests {
    use crate::consts::{self, };
    use crate::info::{self, InfoConfig};
    use crate::Config;
    use crate::output::{OutputType, Output};

    #[test]
    fn info_run_version() {
        let version_output = match consts::VERSION {
            Some(version_string) => {
                Output::new(None, vec!(version_string), OutputType::Info)
            },
            None => {Output::new(None, vec!(consts::UNKNOWN_VERSION), OutputType::Info)}
        };
        assert_eq!(info::run(InfoConfig::VERSION), version_output);
    }

    #[test]
    fn greprs_run_version() {
        let greprs_config = Config{info_config: Some(InfoConfig::VERSION), search_config:None};
        let run_result = super::super::super::run(greprs_config).unwrap();
        assert_eq!((), run_result);
    }
}