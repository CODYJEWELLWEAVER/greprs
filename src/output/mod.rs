use std::collections::HashMap;
use crate::SearchConfig;

/**
Contains output information for a run of greprs
* search_config: Optional search configuration if a search was run
* output_content: A mapping of file names to output lines.
* output_type: OutputType variant
 */
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Output<'a> {
    pub search_config: Option<&'a SearchConfig<'a>>,
    pub output_content: HashMap<String, Vec<Box<String>>>,
    pub output_type: OutputType
}

/**
Encodes possible output options.
* Info: Does not print search configuration.
* Search: Prints search configuration and search results.
* SearchCount: Prints search configuration and match count results.
 */
#[derive(Debug)]
#[derive(PartialEq)]
pub enum OutputType {
    Info,
    Search,
    SearchCount,
}

impl Output<'_> {
    /** Output Constructor */
    pub fn new<'a>(
        search_config: Option<&'a SearchConfig<'a>>,
        output_content: HashMap<String, Vec<Box<String>>>,
        output_type: OutputType
    ) -> Output<'a> {
        return Output {
            search_config,
            output_content,
            output_type
        }
    }

    /** Displays the output contained in this
        Output
     */
    pub fn display(self) {
        match self.output_type {
            OutputType::Info => {
                print_output(&self.output_content, false);
            },
            OutputType::Search |
            OutputType::SearchCount => {
                if let Some(cfg) = self.search_config {
                    print_search_config(cfg);
                }
                let count_lines: bool = self.output_type == OutputType::SearchCount;
                print_output(&self.output_content, count_lines);
            }
        };
    }
}

/**
 Given a mapping of file names and output lines prints the output,
 if more than one file is present then the file names are 
 prepended to the output lines.
 * output_content: mapping of file names to output lines
 * count_lines: true if -c or -count is passed in, prints output lines
 */
fn print_output(
    output_content: &HashMap<String, Vec<Box<String>>>,
    count_lines: bool
) {
    let files_with_ouput = output_content.len();
    let file_name_keys = output_content.keys();
    let mut file_num = 0;

    for file_name in file_name_keys {
        if let Some(file_output) = output_content.get(file_name) {
            if !count_lines {
                // print output lines for each file
                for file_line in file_output {
                    if output_content.len() == 1usize {
                        println!("{}", file_line);
                    } else {
                        // only show file name info when output from multiple 
                        // files is present
                        println!("<{}>\t{}", file_name, file_line)
                    };
                }
            } else {
                // print match counts for each file
                println!("Matching lines in {}: {}", file_name, file_output.len());
            }
            if files_with_ouput > 1 && file_num < files_with_ouput - 1 {
                // print empty lines to deliniate output for 
                // multiple file outputs
                if count_lines || file_output.len() > 1 {
                    println!();
                }
                file_num += 1;
            }
        }
    }
}

/** Print Search Configuration Details */
fn print_search_config(search_config: &SearchConfig) {
    if search_config.queries.len() == 1 {
        // Removes surrounding brackets with 1 query
        println!("Searching for: {}", search_config.queries[0]);
    } else {
        println!("Searching for: {:?}", search_config.queries);
    }
    print!("In: ");
    for file_name in &search_config.files[..] {
        print!("<{}> ", file_name);
    }
    // ends files line and prints empty line
    println!("\n");
}