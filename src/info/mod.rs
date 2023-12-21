use crate::config::InfoConfig;
use crate::consts;
use crate::output::{Output, OutputType};

mod test;

/* Run a InfoConfig. 
   Returns a vector of output strings. */
pub fn run(info_config: InfoConfig) -> Output<'static> {
    let output_lines = match info_config {
        InfoConfig::HELP => {
            let help_output: &'static str = consts::HELP_INFORMATION_OUTPUT;

            vec!(help_output)
        },
        InfoConfig::VERSION => {
            let version_output: &'static str = match consts::VERSION {
                Some(version_string) => version_string,
                None => {consts::UNKNOWN_VERSION}
            };

            vec!(version_output)
        }
    };

    return Output::new(None, output_lines,
        OutputType::INFO)
}