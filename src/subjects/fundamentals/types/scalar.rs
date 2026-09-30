pub fn menu() {
    loop {
        println!();
        println!("── Scalar Types ──────────────────────────────────");

        println!("[1] Signed Integers");
        println!("[2] Unsigned Integers");
        println!("[3] Floating-Point");
        println!("[4] Boolean");
        println!("[5] Character");
        println!("[0] Back");

        let choice = crate::utils::read_input("Select: ");

        match choice.as_str() {
            "1" => crate::subjects::fundamentals::types::integers::i_integers(),
            "2" => crate::subjects::fundamentals::types::unsigned_integers::u_integers(),
            "3" => crate::subjects::fundamentals::types::floats::floats(),
            "4" => crate::subjects::fundamentals::types::booleans::booleans(),
            "5" => crate::subjects::fundamentals::types::chars::chars(),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}
