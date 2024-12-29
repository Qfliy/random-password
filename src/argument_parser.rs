use std::env;
use crate::color_print::{print_yellow_text, print_red_text};

const DEFAULT_PASSWORD_LENGTH: usize = 8;
const DEFAULT_COUNT_OF_PASSWORDS: usize = 1;

const ALERT_INVALID_PARSE_LENGTH: &str = "Dont valid argument length";
const ALERT_INVALID_PARSE_COUNT: &str = "Dont valid argument count";

const HELP_MESSAGE: &str = r#"
Random password generator commands:
    --help or -h -> Help
    --version or -v -> Get version
    number1(default = 8), number2(default = 1)
        -> get number2-th password with number1-th symbols\n
"#;

const VERSION: &str = "0.2.0\n";

pub enum ProgramStatus {
    Generate{length:usize, count:usize},
    Other,
}

pub struct ArgumentParser {
    arguments: Vec<String>,
}

impl ArgumentParser {
    pub fn new() -> ArgumentParser {
        ArgumentParser {
            arguments: env::args().collect(),
        }
    }

    pub fn parse(&self) -> ProgramStatus {
        match self.get_argument(1) {
            "--version" | "-v"=> self.log_and_exit(VERSION),
            "--help" | "-h" => self.log_and_exit(HELP_MESSAGE),
            first_number => self.parse_numbers(&first_number)
        }
    }

    fn get_argument(&self, index: usize) -> &str {
        match self.arguments.get(index) {
            Some(value) => value.as_str(),
            None => "",
        }
    }

    fn log_and_exit(&self, text: &str) -> ProgramStatus {
        print_yellow_text(text);
        ProgramStatus::Other
    }

    fn parse_numbers(&self, first_argument: &str) -> ProgramStatus {
        ProgramStatus::Generate{
            length: self.parse_argument_numbers(
                first_argument,
                ALERT_INVALID_PARSE_LENGTH, DEFAULT_PASSWORD_LENGTH
            ),
            count: self.parse_argument_numbers(
                self.get_argument(2),
                ALERT_INVALID_PARSE_COUNT, DEFAULT_COUNT_OF_PASSWORDS
            )
        }
    }

    fn parse_argument_numbers(
        &self, argument: &str, error_alert: &str, default_value: usize
    ) -> usize {
        match argument.parse::<usize>() {
            Ok(number) => number,
            Err(_) => {
                if argument != "" {
                    print_red_text(error_alert);
                    print_yellow_text("\nusing default value\n"); 
                }

                default_value
            }
        }
    }
}
