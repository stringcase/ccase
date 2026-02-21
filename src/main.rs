use ccase::{CaseOption, PatternOption};
use clap::ArgMatches;
use convert_case::{Boundary, Converter};
use std::env;
use std::io::{self, Read};

fn main() {
    let mut app = ccase::build_app();

    let missing_error = app.error(
        clap::error::ErrorKind::MissingRequiredArgument,
        "The following required arguments were not provided:\n  \
            \x1b[32m<input>...\x1b[m",
    );

    let args = get_args_with_stdin();

    let matches = app.get_matches_from(args);

    let inputs = match matches.get_many::<String>("input") {
        None => {
            if atty::isnt(atty::Stream::Stdin) {
                Default::default()
            } else {
                missing_error.exit();
            }
        }
        Some(inputs) => inputs,
    };

    /*
    inputs.for_each(|input| {
        println!("{:?}", input);
        convert(&matches, input)
    });
    */
    inputs.for_each(|input| convert(&matches, input));
}

fn get_args_with_stdin() -> Vec<String> {
    let mut args: Vec<String> = env::args_os().map(|x| x.into_string().unwrap()).collect();

    if atty::isnt(atty::Stream::Stdin) {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let mut v = Vec::new();
        handle.read_to_end(&mut v).unwrap();

        let s = String::from_utf8(v).unwrap();

        if !s.is_empty() {
            for word in s.lines() {
                args.push(word.trim_end().to_string());
            }
        }
    }

    args
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

#[cfg(test)]
mod test {
    use assert_cmd::{assert::Assert, Command};
    use predicates::str::contains;

    fn ccase(args: &[&str]) -> Assert {
        Command::cargo_bin("ccase").unwrap().args(args).assert()
    }

    #[test]
    fn to_case() {
        ccase(&["-t", "snake", "myVarName"])
            .success()
            .stdout("my_var_name\n");
        ccase(&["--to", "kebab", "myVarName"])
            .success()
            .stdout("my-var-name\n");
        ccase(&["--to", "kebab", "my Var Name"])
            .success()
            .stdout("my-var-name\n");
    }

    #[test]
    fn from_case() {
        ccase(&["-f", "snake", "-t", "pascal", "my_var-name"])
            .success()
            .stdout("MyVar-name\n");
        ccase(&["-t", "snake", "--from", "pascal", "myVar-name"])
            .success()
            .stdout("my_var-name\n");
        ccase(&["-t", "snake", "--from", "lower", "my Var-name"])
            .success()
            .stdout("my_var-name\n");
    }

    #[test]
    fn to_required() {
        ccase(&["myvarname"])
            .failure()
            .stderr(contains("following required arguments"))
            .stderr(contains("--to"));
    }

    #[test]
    fn pattern_only() {
        ccase(&["-p", "capital", "MY_VAR_NAME"])
            .success()
            .stdout("MyVarName\n");
        ccase(&["-p", "sentence", "MY_VAR_NAME"])
            .success()
            .stdout("Myvarname\n");
    }

    #[test]
    fn to_exclusive_with_pattern_delim() {
        ccase(&["-t", "snake", "-p", "capital", "MY_VAR_NAME"])
            .failure()
            .stderr(contains("--to <case>"))
            .stderr(contains("cannot be used with"))
            .stderr(contains("--pattern <pattern>"));
        ccase(&["-t", "snake", "-d", "-", "MY_VAR_NAME"])
            .failure()
            .stderr(contains("--to <case>"))
            .stderr(contains("cannot be used with"))
            .stderr(contains("--delimeter <string>"));
    }

    #[test]
    fn delimeter() {
        ccase(&["-p", "sentence", "-d", ".", "myVarName"])
            .success()
            .stdout("My.var.name\n");
    }

    #[ignore] // atty is tricked in test, look at ccase -t snake manually
    #[test]
    fn input_required() {
        ccase(&["-t", "snake"])
            .failure()
            .stderr(contains("following required arguments"))
            .stderr(contains("input"));
    }

    #[test]
    fn help_default() {
        ccase(&[]).failure().stderr(contains("Usage"));
    }

    #[test]
    fn invalid_case() {
        ccase(&["-t", "snek", "myVarName"])
            .failure()
            .stderr(contains("isn't a valid value"))
            .stderr(contains("--to"));
        ccase(&["-t", "snake", "-f", "snek", "my-varName"])
            .failure()
            .stderr(contains("isn't a valid value"))
            .stderr(contains("--from"));
    }

    #[test]
    fn invalid_pattern() {
        ccase(&["-p", "sent", "myVarName"])
            .failure()
            .stderr(contains("isn't a valid value"))
            .stderr(contains("--pattern"));
        ccase(&["-p", "sent", "-f", "snake", "my-varName"])
            .failure()
            .stderr(contains("isn't a valid value"))
            .stderr(contains("--pattern"));
    }

    #[test]
    fn empty_string_input() {
        ccase(&["-t", "snake", r#""#]).success().stdout("\n");
    }

    #[test]
    fn boundaries() {
        ccase(&["-t", "snake", "-b", "aA", "myVar-Name-Longer"])
            .success()
            .stdout("my_var-name-longer\n");
        ccase(&["-t", "snake", "-b", "-", "myVar-Name-Longer"])
            .success()
            .stdout("myvar_name_longer\n");
    }

    #[test]
    fn from_and_boundaries_exclusive() {
        ccase(&["-t", "snake", "-b", "_", "-f", "kebab", "myVar-Name-Longer"])
            .failure()
            .stderr(contains("--from"))
            .stderr(contains("cannot be used with"))
            .stderr(contains("--boundaries"));
    }

    #[test]
    fn multiple_inputs() {
        ccase(&["-t", "snake", "myVarName", "anotherMultiWordToken"])
            .success()
            .stdout("my_var_name\nanother_multi_word_token\n");
    }

    mod stdin {
        use super::*;

        fn pipe_ccase(stdin: &str, args: &[&str]) -> Assert {
            Command::cargo_bin("ccase")
                .unwrap()
                .args(args)
                .write_stdin(stdin)
                .assert()
        }

        #[test]
        fn stdin() {
            pipe_ccase("myVarName", &["-t", "snake"])
                .success()
                .stdout("my_var_name\n");
        }

        #[test]
        fn newline_ending() {
            pipe_ccase("myVarName\n", &["-t", "snake"])
                .success()
                .stdout("my_var_name\n");
        }

        #[test]
        fn empty() {
            pipe_ccase(r#""#, &["-t", "snake"]).success().stdout("");
        }

        #[test]
        fn multiple_inputs() {
            pipe_ccase("myVarName\nanotherMultiWordToken\n", &["-t", "pascal"])
                .success()
                .stdout("MyVarName\nAnotherMultiWordToken\n");
        }
    }
}
