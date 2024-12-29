
const CONSOLE_COLOR_RED: &str = "\x1b[31m";
const CONSOLE_COLOR_YELLOW: &str = "\x1b[33m";
const CONSOLE_STYLE_RESET: &str = "\x1b[0m";

macro_rules! print_with_color {
    ($color_value:expr, $text:expr) => {
        print!("{}{}{}", $color_value, $text, CONSOLE_STYLE_RESET);
    };
}

pub fn print_red_text(text: &str) {
    print_with_color!(CONSOLE_COLOR_RED, text);
}

pub fn print_yellow_text(text: &str) {
    print_with_color!(CONSOLE_COLOR_YELLOW, text);
}

