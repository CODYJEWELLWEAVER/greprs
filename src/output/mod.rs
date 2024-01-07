use std::collections::HashMap;
use crate::consts;
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
    pub search_config: Option<SearchConfig<'a>>,
    pub output_content: HashMap<String, Vec<String>>,
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
        search_config: Option<SearchConfig<'a>>,
        output_content: HashMap<String, Vec<String>>,
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
                    print_search_config(&cfg);
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
    output_content: &HashMap<String, Vec<String>>,
    count_lines: bool
) {
    let files_with_ouput = output_content.len();
    let mut file_num = 0;

    output_content.iter().for_each( |(file_name, file_output)| {
        if !count_lines {
            file_output.iter().for_each(|line| {
                if output_content.len() == 1usize {
                    println!("{}", line);
                } else {
                    // only show file name info when output from multiple 
                    // files is present
                    println!("<{}>\t{}", file_name, line);
                };
            });
        } else {
            // print match counts for each file
            println!("Matching lines in {}: {}", file_name, file_output.len());
        }
        // print empty lines to deliniate output for 
        // multiple file outputs, EXCEPT after last file
        if file_output.len() > 0  && file_num != files_with_ouput - 1{
            println!();
        }
        file_num += 1;
    });
    if files_with_ouput == 0 {
        println!("{}", consts::ERR_MSG_NO_MATCHES);
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
    search_config.files.iter().for_each(|file_name| {
        print!("<{}> ", file_name);
    });
    // ends files line and prints empty line
    println!("\n");
}