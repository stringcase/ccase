use ccase::{CaseOption, PatternOption};
use clap::ArgMatches;
use convert_case::{Boundary, Converter};
use std::io::{self, IsTerminal, Read};

fn main() {
    let app = ccase::build_app();
    let matches = app.get_matches();

    let inputs: Vec<String> = match matches.get_many::<String>("input") {
        Some(inputs) => inputs.cloned().collect(),
        None => {
            // No command-line inputs - try stdin if not a terminal
            if !io::stdin().is_terminal() {
                read_stdin_lines()
            } else {
                // No input and stdin is terminal - show error
                eprintln!("error: missing required argument: <input>...");
                std::process::exit(1);
            }
        }
    };

    for input in &inputs {
        convert(&matches, input);
    }
}

fn read_stdin_lines() -> Vec<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_to_string(&mut buf).unwrap();
    buf.lines().map(|s| s.trim_end().to_string()).collect()
}

fn convert(matches: &ArgMatches, input: &String) {
    // check if from or boundaries or none

    let mut conv = Converter::new();

    if let Some(&from) = matches.get_one::<CaseOption>("from") {
        // --from
        conv = conv.from_case(from.to_case());
    } else if let Some(boundary_str) = matches.get_one::<String>("boundaries") {
        // --boundaries
        let boundaries = Boundary::defaults_from(boundary_str.as_str());
        conv = conv.set_boundaries(&boundaries);
    }

    if let Some(&to) = matches.get_one::<CaseOption>("to") {
        // --to
        conv = conv.to_case(to.to_case());
    } else if let Some(&pattern) = matches.get_one::<PatternOption>("pattern") {
        // --pattern
        conv = conv.set_pattern(pattern.to_pattern());

        if let Some(delim) = matches.get_one::<String>("delimeter") {
            // --delimeter
            conv = conv.set_delimiter(delim);
        }
    }

    println!("{}", conv.convert(input))
}
