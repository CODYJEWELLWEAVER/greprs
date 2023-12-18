use crate::config::InfoConfig;
use crate::consts;

mod test;

/* Run a InfoConfig. 
   Returns a vector of output strings. */
pub fn run(info_config: InfoConfig) -> Vec<&'static str>{
    return match info_config {
        InfoConfig::HELP => {
            let help_output: &str = consts::HELP_INFORMATION_OUTPUT;

            vec!(help_output)
        },
        InfoConfig::VERSION => {
            let version_output: &str = match consts::VERSION {
                Some(version_string) => version_string,
                None => {consts::UNKNOWN_VERSION}
            };

            vec!(version_output)
        }
    }
}