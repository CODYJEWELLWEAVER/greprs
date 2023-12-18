#[cfg(test)]
pub mod info_tests {
    use crate::consts::{UNKNOWN_VERSION, self, };
    use crate::info::{self, InfoConfig};
    use crate::Config;

    #[test]
    fn info_run_version() {
        let version_output: &str = match consts::VERSION {
            Some(version_string) => {
                version_string
            },
            None => {UNKNOWN_VERSION}
        };
        assert_eq!(info::run(InfoConfig::VERSION), [version_output]);
    }

    #[test]
    fn greprs_run_version() {
        let greprs_config = Config{info_config: Some(InfoConfig::VERSION), search_config:None};
        let run_result = super::super::super::run(greprs_config).unwrap();
        assert_eq!((), run_result);
    }
}