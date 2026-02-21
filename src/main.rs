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

    #[ignore] // Can't test TTY behavior - in test env stdin is always piped, not TTY
    #[test]
    fn input_required_tty() {
        // When stdin is a TTY and no input provided, should show error.
        // This can only be verified manually: `ccase -t snake`
        ccase(&["-t", "snake"])
            .failure()
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

        #[test]
        fn cli_input_ignores_stdin() {
            // When CLI input is provided, stdin should be ignored.
            // This fixes the bug where ccase blocks in a while-read loop:
            //   printf "a\nb\n" | while read word; do ccase -t upper "$word"; done
            // See: https://github.com/rutrum/ccase/issues/3
            pipe_ccase("ignored_stdin_content", &["-t", "upper", "hello"])
                .success()
                .stdout("HELLO\n");
        }
    }
}
