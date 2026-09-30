use crate::utils::{bullet, code, header, info, remember, section};

pub fn comments() {
    header("COMMENTS");

    introduction();
    line_comments();
    block_comments();
    documentation_comments();
    when_to_use_comments();
    summary();
}

fn introduction() {
    section("What is a comment?");

    info("A comment is text written for humans that Rust does not execute.");

    println!();

    code("// This is a comment");

    println!();

    bullet("Comments can explain code.");
    bullet("Comments are ignored by the compiler.");
    bullet("Comments can make unfamiliar code easier to understand.");

    remember("Comments are for humans, not for program execution.");
}

fn line_comments() {
    section("Line comments");

    code("// This entire line is a comment");
    code("let age = 22; // This explains the value");

    println!();

    let age = 22;

    info(&format!("The program still executes: age = {age}"));

    println!();

    bullet("A line comment starts with //.");
    bullet("Everything after // on that line is ignored.");

    remember("// creates a line comment.");
}

fn block_comments() {
    section("Block comments");

    code("/*");
    code("   This is a");
    code("   block comment.");
    code("*/");

    println!();

    info("Block comments can span multiple lines.");

    println!();

    bullet("Block comments start with /*.");
    bullet("Block comments end with */.");
    bullet("Everything between them is ignored by the compiler.");

    remember("Use /* ... */ when a comment needs to span multiple lines.");
}

fn documentation_comments() {
    section("Documentation comments");

    code("/// Explains the item that follows.");
    code("fn add(a: i32, b: i32) -> i32 {");
    code("    a + b");
    code("}");

    println!();

    info("Documentation comments are used to describe Rust items.");
    info("They can be processed by Rust's documentation tools.");

    println!();

    bullet("/// documents the item that follows.");
    bullet("// is an ordinary comment.");
    bullet("//! documents the enclosing module or crate.");

    remember("Documentation comments are part of Rust's documentation system.");
}

fn when_to_use_comments() {
    section("When should you use comments?");

    println!("Good reasons:");

    bullet("Explain why something is done.");
    bullet("Explain non-obvious behavior.");
    bullet("Document assumptions or constraints.");
    bullet("Provide context that the code itself cannot express clearly.");

    println!();

    println!("Avoid comments that simply repeat the code:");

    code("let age = 22; // Set age to 22");

    println!();

    info("The code already makes that obvious.");

    remember(
        "Prefer clear code first; use comments to explain what the code cannot explain by itself.",
    );
}

fn summary() {
    section("Quick reference");

    bullet("// creates a line comment.");
    bullet("/* ... */ creates a block comment.");
    bullet("/// creates documentation for the following item.");
    bullet("//! documents a module or crate.");
    bullet("Comments are ignored during program execution.");
    bullet("Useful comments explain intent, context, or non-obvious behavior.");

    println!();

    remember("Good comments explain why, not merely what.");
}
