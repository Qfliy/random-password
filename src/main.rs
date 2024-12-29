
mod argument_parser;
mod color_print;
mod password_generator;
mod password_displayer;

use argument_parser::{ArgumentParser, ProgramStatus};
use password_generator::PasswordGenerator;
use password_displayer::disply_passwords;

fn start_program(length: usize, count: usize) {
    disply_passwords(
        PasswordGenerator::new(
            length, count
        ).generate_passwords(),
        count
    )
}

fn main() {
    let parser = ArgumentParser::new();
    
    if let ProgramStatus::Generate{length, count} = parser.parse() {
        start_program(length, count)
    }
}
