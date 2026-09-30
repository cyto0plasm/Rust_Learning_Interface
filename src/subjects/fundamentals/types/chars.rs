use crate::utils::{bullet, code, header, info, remember, section};

pub fn chars() {
    header("CHARACTER");

    introduction();
    declarations();
    unicode();
    char_vs_string();
    methods();
    summary();
}

fn introduction() {
    section("What is char?");

    info("Rust's char type represents a single Unicode scalar value.");

    println!();

    code("let letter: char = 'A';");
    code("let arabic: char = 'م';");
    code("let emoji: char = '🦀';");

    println!();

    remember("A Rust char is 4 bytes and represents a Unicode scalar value.");
}

fn declarations() {
    section("Declaring characters");

    let letter: char = 'A';
    let number: char = '7';
    let symbol: char = '#';

    code("let letter: char = 'A';");
    code("let number: char = '7';");
    code("let symbol: char = '#';");

    println!();

    info(&format!("letter = {letter}"));
    info(&format!("number = {number}"));
    info(&format!("symbol = {symbol}"));
}

fn unicode() {
    section("Unicode");

    let arabic = 'م';
    let japanese = '日';
    let emoji = '🦀';

    println!("Rust chars support Unicode:");

    println!();

    println!("  Arabic   → {arabic}");
    println!("  Japanese → {japanese}");
    println!("  Emoji    → {emoji}");

    println!();

    info("char is not limited to ASCII characters.");
}

fn char_vs_string() {
    section("char vs String");

    code("'A'      → char");
    code("\"A\"      → string slice");
    code("\"Hello\"  → string slice");

    println!();

    bullet("Single quotes represent a char.");
    bullet("Double quotes represent a string.");

    remember("char represents one Unicode scalar value, not a whole string.");
}

fn methods() {
    section("Useful char methods");

    let character = 'A';

    println!("Given:");
    println!("  character = {character}");

    println!();

    println!("  is_alphabetic() → {}", character.is_alphabetic());
    println!("  is_numeric()    → {}", character.is_numeric());
    println!("  is_uppercase()  → {}", character.is_uppercase());
    println!("  is_lowercase()  → {}", character.is_lowercase());

    println!();

    bullet("Character methods can inspect what kind of character you have.");
}

fn summary() {
    section("Quick reference");

    bullet("char represents one Unicode scalar value.");
    bullet("char uses 4 bytes.");
    bullet("char uses single quotes.");
    bullet("char supports Unicode.");
    bullet("Strings use double quotes.");

    println!();

    remember(
        "char is one character-like Unicode value; String/&str represent sequences of characters.",
    );
}
