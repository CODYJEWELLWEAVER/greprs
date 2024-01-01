use crate::config::InfoConfig;
use crate::consts;
use crate::output::{Output, OutputType};
use std::collections::HashMap;

mod test;

static HELP_KEY: &str = "help";
static VERSION_KEY: &str = "version";

/** 
   Run a InfoConfig. 
   Returns an Output containing selected info option ouptut. 
 */
pub fn run(info_config: InfoConfig) -> Output<'static> {
    let mut output_content = HashMap::new();
    let output_lines: Vec<Box<String>> = match info_config {
        InfoConfig::Help => {
            let help_output = consts::HELP_INFORMATION_OUTPUT;

            vec!(Box::new(help_output.to_string()))
        },
        InfoConfig::Version => {
            let version_output: &str = match consts::VERSION {
                Some(version_string) => version_string,
                None => {consts::UNKNOWN_VERSION}
            };

            vec!(Box::new(version_output.to_string()))
        }
    };

    match info_config {
        InfoConfig::Help => output_content.insert(HELP_KEY.to_string(), output_lines),
        InfoConfig::Version => output_content.insert(VERSION_KEY.to_string(), output_lines)
    };

    return Output::new(None, output_content,
        OutputType::Info);
}