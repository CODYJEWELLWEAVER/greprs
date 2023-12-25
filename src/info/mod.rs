use crate::config::InfoConfig;
use crate::consts;
use crate::output::{Output, OutputType};

mod test;

/* Run a InfoConfig. 
   Returns a vector of output strings. */
pub fn run(info_config: InfoConfig) -> Output<'static> {
    let output_lines: Vec<Box<String>> = match info_config {
        InfoConfig::HELP => {
            let help_output = consts::HELP_INFORMATION_OUTPUT;

            vec!(Box::new(help_output.to_string()))
        },
        InfoConfig::VERSION => {
            let version_output: &str = match consts::VERSION {
                Some(version_string) => version_string,
                None => {consts::UNKNOWN_VERSION}
            };

            vec!(Box::new(version_output.to_string()))
        }
    };

    return Output::new(None, output_lines,
        OutputType::Info);
}