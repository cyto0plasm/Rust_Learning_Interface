use crate::utils::{header, read_input};

pub fn menu() {
    loop {
        header("COMPOUND TYPES");

        println!("[1] Tuples");
        println!("[2] Arrays");
        println!("[0] Back");

        let choice = read_input("Select: ");

        match choice.as_str() {
            "1" => super::tuples::tuples(),
            "2" => super::arrays::arrays(),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}
