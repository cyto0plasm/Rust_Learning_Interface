use crate::utils::{bullet, code, header, info, remember, section};

pub fn expressions() {
    header("STATEMENTS & EXPRESSIONS");

    introduction();
    statements();
    expressions_section();
    expression_values();
    blocks();
    semicolons();
    summary();
}

fn introduction() {
    section("Statements vs expressions");

    info("Rust code is built from statements and expressions.");
    info("Understanding the difference is important because expressions produce values.");

    println!();

    bullet("Statements perform an action.");
    bullet("Expressions evaluate to a value.");

    remember("Statements do something; expressions produce something.");
}

fn statements() {
    section("Statements");

    code("let x = 5;");

    println!();

    let x = 5;

    info(&format!("x = {x}"));

    println!();

    bullet("A let declaration is a statement.");
    bullet("Statements generally do not return a value.");

    info("The semicolon marks the end of the let statement.");
}

fn expressions_section() {
    section("Expressions");

    code("5 + 3");
    code("x * 2");
    code("if true { 10 } else { 20 }");

    println!();

    let a = 5 + 3;
    let b = a * 2;

    info(&format!("5 + 3 = {a}"));
    info(&format!("a * 2 = {b}"));

    println!();

    bullet("Arithmetic operations are expressions.");
    bullet("A function call is an expression.");
    bullet("An if expression produces a value.");

    remember("Expressions evaluate to values.");
}

fn expression_values() {
    section("Using expression values");

    let result = {
        let first = 10;
        let second = 20;

        first + second
    };

    code("let result = {");
    code("    let first = 10;");
    code("    let second = 20;");
    code("");
    code("    first + second");
    code("};");

    println!();

    info(&format!("result = {result}"));

    println!();

    info("The block evaluates to the value of its final expression.");

    remember("An expression can be used wherever Rust expects a value.");
}

fn blocks() {
    section("Blocks are expressions");

    let value = {
        let number = 5;

        println!("  Inside block: number = {number}");

        number + 1
    };

    println!();

    info(&format!("Block returned = {value}"));

    println!();

    bullet("A block can contain multiple statements.");
    bullet("Its final expression becomes the block's value.");
    bullet("The final expression does not use a semicolon.");

    remember("A block can calculate a value and return it to the surrounding code.");
}

fn semicolons() {
    section("Semicolons matter");

    let without_semicolon = { 5 };

    let with_semicolon = {
        5;
    };

    code("{");
    code("    5");
    code("}");

    println!();

    info(&format!("Without semicolon → {without_semicolon}"));

    println!();

    code("{");
    code("    5;");
    code("}");

    println!();

    info(&format!("With semicolon → {:?}", with_semicolon));

    println!();

    bullet("Without the semicolon, 5 is the block's final expression.");
    bullet("With the semicolon, the expression becomes a statement.");
    bullet("The block therefore evaluates to ().");

    remember("A semicolon can change an expression into a statement.");
}

fn summary() {
    section("Quick reference");

    bullet("Statements perform actions.");
    bullet("Expressions produce values.");
    bullet("Arithmetic operations are expressions.");
    bullet("Function calls are expressions.");
    bullet("Blocks are expressions.");
    bullet("The final expression in a block becomes its value.");
    bullet("A semicolon turns an expression into a statement.");

    println!();

    remember("Rust is expression-oriented: many constructs produce values.");
}
