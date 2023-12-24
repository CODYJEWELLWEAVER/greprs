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
    let file_contents: Vec<String> = open_files(&search_config.files)?;

    let mut search_results = Vec::new();
    let match_pattern = matcher::MatchPattern::new(search_config)?;

    for file in file_contents {
        for line in file.clone().lines() {
            if match_pattern.matches(line) {
                search_results.push(Box::new(line.to_string()));
            }
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

fn open_files(files: &Vec<& str>) 
-> Result<Vec<String>, Box<dyn Error>> {
    let mut stderr = std::io::stderr();

    let mut file_contents: Vec<String> = Vec::new();

    for file_name in files.iter() {
        
        if let Ok(mut file) = File::open(file_name) {
            if let Ok(content_string) = io::read_to_string(&mut file) {
                file_contents.push(content_string);
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

    return match file_contents.len() {
        0 => {
            Err(Box::from("Could not open any files!"))
        },
        _ => {
            Ok(file_contents)
        }
    };
}