use std::io::{self, Write};

pub fn header(title: &str) {
    println!();
    println!("\x1b[1;34m╔══════════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[1;34m║ {:^48} ║\x1b[0m", title);
    println!("\x1b[1;34m╚══════════════════════════════════════════════════╝\x1b[0m");
}

pub fn section(title: &str) {
    println!();
    println!(
        "\x1b[1;36m── {} ────────────────────────────────────────────\x1b[0m",
        title
    );
}

pub fn info(text: &str) {
    println!("  {}", text);
}

pub fn bullet(text: &str) {
    println!("  • {}", text);
}

pub fn code(text: &str) {
    println!("    {}", text);
}

pub fn divider() {
    println!("\x1b[1;34m──────────────────────────────────────────────────\x1b[0m");
}

pub fn note(text: &str) {
    println!();
    println!("\x1b[1;33mNOTE:\x1b[0m {}", text);
}

pub fn remember(text: &str) {
    println!();
    println!("\x1b[1;32mREMEMBER:\x1b[0m {}", text);
}

pub fn read_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_lowercase()
}
