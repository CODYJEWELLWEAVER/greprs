use crate::config::search_config::SearchConfig;
use crate::{matcher, consts};
use crate::output::{Output, OutputType};
use std::error::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, self};

mod test;

const NO_OPEN_FILES: usize = 0;

/**
Runs the search given the config parameter
and returns the output list.
* param: &SearchConfig - search config parsed from cli args
* return: output of search on success, boxed error on failure
*/
pub fn run<'a>(
    search_config: SearchConfig<'a>
) -> Result<Output<'_>, Box<dyn Error>> {
    let open_files: Vec<OpenFile> = open_files(&search_config.files)?;

    let mut output_content = HashMap::new();

    let match_pattern = matcher::MatchPattern::new(&search_config)?;

    open_files.iter().for_each(|file| {
        let mut search_results = Vec::new();

        file.contents.lines().for_each(|line| {
            if match_pattern.matches(line) {
                // Moves output lines to heap to be returned
                // let boxed_line_string = Box::new(line.to_string());
                search_results.push(line.to_string());
            }
        });

        if search_results.len() > 0 {
            output_content.insert(file.name.to_string(), search_results);
        }
    });

    let output_type = if search_config.count_output {
        OutputType::SearchCount
    } else {
        OutputType::Search
    };

    let search_output = Output::new(
        Some(search_config),
        output_content,
        output_type
    );

    Ok(search_output)
}

/**
Associates a content string with a file name
*/
struct OpenFile<'a> {
    name: &'a str,
    contents: String
}

impl OpenFile<'_> {
    /** creates new OpenFile */
    pub fn new(name: &str, contents: String) -> OpenFile<'_> {
        return OpenFile {
            name,
            contents
        }
    }
}

/**
Given a vector of file names, attempts to open them and returns 
OpenFile objects corresponding to each successfully opened file.
* note: Wont return an error if some file names passed are invalid if at least 
one file name is valid. If a file cannot be opened and stderr cannot be written to 
will return an error.
* param: &Vec<& str> Vector of file names.
* return: Vector of OpenFile objects or error
 */
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
        NO_OPEN_FILES => {
            // return error if no file names are valid
            Err(Box::from(consts::ERR_MSG_NO_OPEN_FILES))
        },
        _ => {
            Ok(open_files)
        }
    };
}