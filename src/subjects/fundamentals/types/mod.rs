pub mod compound;
pub mod scalar;

pub mod arrays;
pub mod booleans;
pub mod chars;
pub mod floats;
pub mod integers;
pub mod tuples;
pub mod unsigned_integers;

use crate::utils::{header, read_input};

pub fn types_menu() {
    loop {
        header("DATA TYPES");

        println!("[1] Scalar Types");
        println!("[2] Compound Types");
        println!("[0] Back");

        let choice = read_input("Select: ");

        match choice.as_str() {
            "1" => scalar::menu(),
            "2" => compound::menu(),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}
