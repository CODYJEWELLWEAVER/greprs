use crate::config::search_config::SearchConfig;
use crate::matcher;
use crate::output::{Output, OutputType};
use std::error::Error;
use std::fs::File;
use std::io::{Write, self};

mod test;

// Runs the search given the config parameter
// and returns the output list.
pub fn run<'a>(
    search_config: &'a SearchConfig<'a>
) -> Result<Output<'_>, Box<dyn Error>> {
    let open_files: Vec<OpenFile> = open_files(&search_config.files)?;
    let num_files = &search_config.files.len();

    let mut search_results = Vec::new();
    let match_pattern = matcher::MatchPattern::new(search_config)?;

    for file in open_files {
        for line in file.contents.lines() {
            if match_pattern.matches(line) {
                // Moves output lines to heap to be returned
                let boxed_line_string = match num_files {
                    1 => {
                        Box::new(line.to_string())
                    },
                    _ => {
                        // Append file name to output when searching more 
                        // than one file.
                        Box::new(String::from(format!("<{}>\t", file.name)) + &line)
                    }
                };

                search_results.push(boxed_line_string);
            }
        }
        if *(num_files) > 1 {
            // add file output separator
            search_results.push(Box::new("\n".to_string()))
        }
    }

    let output_type = if search_config.count_output {
        OutputType::SearchCount
    }
    else {
        OutputType::Search
    };

    let search_output = Output::new(
        Some(&search_config),
        search_results,
        output_type
    );

    Ok(search_output)
}

// Associates a content string with a file name
struct OpenFile<'a> {
    name: &'a str,
    contents: String
}

impl OpenFile<'_> {
    pub fn new(name: &str, contents: String) -> OpenFile<'_> {
        return OpenFile {
            name,
            contents
        }
    }
}

fn open_files<'a>(files: &'a Vec<& str>) 
-> Result<Vec<OpenFile<'a>>, Box<dyn Error>> {
    let mut stderr = std::io::stderr();

    let mut open_files: Vec<OpenFile> = Vec::new();

    for file_name in files.iter() {
        if let Ok(mut file) = File::open(file_name) {
            if let Ok(content_string) = io::read_to_string(&mut file) {
                let open_file = OpenFile::new(file_name, content_string);
                open_files.push(open_file);
            } else {
                writeln!(
                    &mut stderr,
                    "Cannot read from: {}",
                    file_name
                ) ?
            }
        } else {
            // Write names of files that cannot be opened to stderr
            writeln!(
                &mut stderr,
                "Cannot open: {}",
                file_name
            ) ? 
        }   
    }

    return match open_files.len() {
        0 => {
            Err(Box::from("Could not open any files!"))
        },
        _ => {
            Ok(open_files)
        }
    };
}