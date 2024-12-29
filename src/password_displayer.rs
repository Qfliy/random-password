use crate::color_print::{print_yellow_text};

pub fn disply_passwords(passwords: Vec<String>, count: usize) {
    if count == 0 { return; }
    
    if count == 1 {
        print_yellow_text("\tYour password: ");
        println!("{}", passwords[0]);
        return
    }

    for i in 0..count {
        print_yellow_text(format!("{}.\tYour password: ", i + 1).as_str());
        println!("{}", passwords[i]);
    }
}
