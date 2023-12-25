use crate::SearchConfig;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Output<'a> {
    pub search_config: Option<&'a SearchConfig<'a>>,
    pub output_lines: Vec<Box<String>>,
    pub output_type: OutputType
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum OutputType {
    Info,
    Search,
    SearchCount,
}

impl Output<'_> {
    pub fn new<'a>(
        search_config: Option<&'a SearchConfig<'a>>,
        output_lines: Vec<Box<String>>,
        output_type: OutputType
    ) -> Output<'a> {
        return Output {
            search_config,
            output_lines,
            output_type
        }
    }

    pub fn display(self) -> () {
        match self.output_type {
            OutputType::Info => {
                print_output(&self.output_lines);
            },
            OutputType::Search => {
                print_search_config(self.search_config.unwrap());
                print_output(&self.output_lines);
            },
            OutputType::SearchCount => {
                print_count(&self.output_lines);
            }
        };
    }
}

fn print_count(lines: &Vec<Box<String>>) {
    println!("Matching Lines: {}", lines.len())
}

fn print_output(lines: &Vec<Box<String>>) {
    for line in lines {
        println!("{}", line);
    }
}

/* Print Search Configuration Details */
fn print_search_config(search_config: &SearchConfig) {
    println!("Searching for: '{:?}'", search_config.queries);
    print!("In: ");
    for file_name in &search_config.files[..] {
        print!("<{}> ", file_name);
    }
    print!("\n");
}