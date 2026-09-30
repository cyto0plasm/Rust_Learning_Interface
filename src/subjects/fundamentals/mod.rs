pub mod comments;
pub mod constants;
pub mod expressions;
pub mod functions;
pub mod shadowing;
pub mod types;
pub mod variables;

use crate::utils::{header, read_input};

pub fn menu() {
    loop {
        header("RUST FUNDAMENTALS");

        println!("[1] Variables & Mutability");
        println!("[2] Constants");
        println!("[3] Shadowing");
        println!("[4] Data Types");
        println!("[5] Functions");
        println!("[6] Statements & Expressions");
        println!("[7] Comments");
        println!("[0] Back / Exit");

        let choice = read_input("Select: ");

        match choice.as_str() {
            "1" => variables::variables(),
            "2" => constants::constants(),
            "3" => shadowing::shadowing(),
            "4" => types::types_menu(),
            "5" => functions::functions(),
            "6" => expressions::expressions(),
            "7" => comments::comments(),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}
